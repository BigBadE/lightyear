use crate::{ClientId, Key, PRIVATE_KEY_BYTES, ServerConfig};
use alloc::{sync::Arc, vec::Vec};
use bevy_app::{App, Plugin, PostUpdate, PreUpdate};
use bevy_ecs::{
    component::Component,
    entity::{Entity, UniqueEntitySlice},
    error::Result,
    observer::Trigger,
    query::{Has, With, Without},
    relationship::RelationshipTarget,
    schedule::IntoScheduleConfigs,
    system::{Commands, ParallelCommands, Query, Res},
};
use bevy_time::{Real, Time};
use lightyear_connection::client::{Connected, Disconnected, Disconnecting};
use lightyear_connection::host::HostClient;
use lightyear_connection::prelude::{server::*, *};
use lightyear_connection::server::Stopping;
use lightyear_core::id::{LocalId, PeerId, RemoteId};
use lightyear_link::prelude::{LinkOf, Server};
use lightyear_link::{Link, LinkSet};
use lightyear_transport::plugin::TransportSet;
use tracing::{error, info, trace};

pub struct NetcodeServerPlugin;

#[derive(Default)]
pub(crate) struct NetcodeServerContext {
    pub(crate) connections: Vec<(ClientId, Entity)>,
    pub(crate) disconnections: Vec<(ClientId, Entity)>,
}

#[derive(Component)]
#[require(Server)]
pub struct NetcodeServer {
    pub(crate) inner: crate::server::Server<NetcodeServerContext>,
}

// TODO: should be part of the NetcodeServer component
#[derive(Debug, Clone)]
pub struct NetcodeConfig {
    pub num_disconnect_packets: usize,
    pub keep_alive_send_rate: f64,
    /// Set the duration (in seconds) after which the server disconnects a client if they don't hear from them.
    /// This is valid for tokens generated by the server.
    /// The default is 3 seconds. A negative value means no timeout.
    pub client_timeout_secs: i32,
    pub protocol_id: u64,
    pub private_key: Key,
}

impl Default for NetcodeConfig {
    fn default() -> Self {
        Self {
            num_disconnect_packets: 10,
            keep_alive_send_rate: 1.0 / 10.0,
            client_timeout_secs: 3,
            protocol_id: 0,
            private_key: [0; PRIVATE_KEY_BYTES],
        }
    }
}

impl NetcodeConfig {
    pub fn with_protocol_id(mut self, protocol_id: u64) -> Self {
        self.protocol_id = protocol_id;
        self
    }
    pub fn with_key(mut self, key: Key) -> Self {
        self.private_key = key;
        self
    }

    pub fn with_client_timeout_secs(mut self, client_timeout_secs: i32) -> Self {
        self.client_timeout_secs = client_timeout_secs;
        self
    }
}

impl NetcodeServer {
    pub fn new(config: NetcodeConfig) -> Self {
        let context = NetcodeServerContext::default();
        let mut cfg = ServerConfig::with_context(context)
            .on_connect(|id, entity, ctx| {
                ctx.connections.push((id, entity));
            })
            .on_disconnect(|id, entity, ctx| {
                ctx.disconnections.push((id, entity));
            });
        cfg = cfg.keep_alive_send_rate(config.keep_alive_send_rate);
        cfg = cfg.num_disconnect_packets(config.num_disconnect_packets);
        cfg = cfg.client_timeout_secs(config.client_timeout_secs);
        let server =
            crate::server::Server::with_config(config.protocol_id, config.private_key, cfg)
                .expect("Could not create server netcode");
        Self { inner: server }
    }
}

impl NetcodeServerPlugin {
    /// Takes packets from the Link, process them through the server,
    /// and buffer them back into the link to be sent by the IO
    fn send(
        mut server_query: Query<(&mut NetcodeServer, &Server), (With<Server>, Without<Stopped>)>,
        client_query: Query<
            (
                Entity,
                &mut Link,
                Option<&RemoteId>,
                Option<&Connected>,
                Option<&Disconnecting>,
            ),
            (With<LinkOf>, Without<HostClient>),
        >,
    ) {
        // TODO: we should be able to do ParIterMut if we can make the code understand
        //  that the transports/links are all mutually exclusive...
        //  Maybe some unsafe Cloneble wrapper around the client_query?
        //  Or maybe store the clients into a Local<Vec<(&mut Transport, &mut Link)>>? so that we can iterate faster through them?
        // we use Arc to tell the compiler that we know that the queries won't be used to access
        // the same clients (because each Link is uniquely associated with a single server)
        // This allow us to iterate in parallel over all servers
        let client_query = Arc::new(client_query);
        server_query
            .par_iter_mut()
            // .iter_mut()
            .for_each(|(mut netcode_server, server)| {
                // SAFETY: we know that each client is unique to a single server so we won't
                //  violate aliasing rules
                let mut client_query = unsafe { client_query.reborrow_unsafe() };

                // SAFETY: we know that the entities of a relationship are unique
                let unique_slice =
                    unsafe { UniqueEntitySlice::from_slice_unchecked(server.collection()) };
                client_query.iter_many_unique_mut(unique_slice).for_each(
                    |(entity, mut link, remote_id, connected, disconnecting)| {
                        // TODO: we can be here while the link has been established, but the client is not yet connected
                        //  so the PeerId is not Netcode! I think we should just error?

                        // If the client was connected, it has a Netcode client_id
                        if connected.is_some() {
                            if let Some(PeerId::Netcode(client_id)) = remote_id.map(|x| x.0) {
                                for _ in 0..link.send.len() {
                                    if let Some(payload) = link.send.pop() {
                                        netcode_server
                                            .inner
                                            .send(payload, client_id, &mut link.send)
                                            .inspect_err(|e| {
                                                error!("Error sending packet: {:?}", e);
                                            })
                                            .ok();
                                    }
                                }

                                // NOTE: we send any netcode packets AFTER the user payloads have been processed
                                // (because we want the
                                netcode_server
                                    .inner
                                    .send_keepalives(client_id, &mut link.send)
                                    .inspect_err(|e| {
                                        error!("Error sending keepalive packet: {:?}", e);
                                    })
                                    .ok();
                            } else {
                                error!(
                                    "The client is Connected but does not have a RemoteId component"
                                );
                            }
                        } else {
                            // if the client is not connected, remove any messages buffered in link.send
                            // We don't want to allow users to send messages while not connected
                            //
                            // However if we are disconnecting, we still want to send the disconnect packets
                            // (we don't use `send_netcode_packets` because we need to remove the client from `send_queue`)
                            if disconnecting.is_none() {
                                link.send.drain();
                            }
                        }

                        // even if it was not connected, we might need to send the netcode packets that were buffered
                        netcode_server
                            .inner
                            .send_netcode_packets(entity, &mut link.send);

                        // #[cfg(feature = "test_utils")]
                        // trace!("SERVER: length of each packet in send: {:?}", link.send.iter().map(|p| p.len()).collect::<Vec<_>>());
                    },
                );
            })
    }

    /// Receive packets from the Link, process them through the server,
    /// then buffer them back into the Link
    fn receive(
        parallel_commands: ParallelCommands,
        real_time: Res<Time<Real>>,
        mut server_query: Query<
            (Entity, &mut NetcodeServer, &mut Server, Has<Stopping>),
            Without<Stopped>,
        >,
        link_query: Query<(Entity, &mut Link), (With<LinkOf>, Without<HostClient>)>,
    ) {
        let delta = real_time.delta();

        // we use Arc to tell the compiler that we know that the queries won't be used to access
        // the same clients (because each Link is uniquely associated with a single server)
        // This allow us to iterate in parallel over all servers
        let link_query = Arc::new(link_query);

        // receive packets from the link and process them through the server
        server_query.par_iter_mut().for_each(
            |(server_entity, mut netcode_server, mut server, stopping)| {
                parallel_commands.command_scope(|mut c| {
                    // SAFETY: we know that each client is unique to a single server so we won't
                    //  violate aliasing rules
                    let mut link_query = unsafe { link_query.reborrow_unsafe() };

                    netcode_server.inner.update_state(delta.as_secs_f64());

                    // TODO: try to make this parallel!
                    // enable split borrows
                    let server = &mut *server;
                    // SAFETY: we know that the list of client entities are unique because it is a Relationship
                    let unique_slice =
                        unsafe { UniqueEntitySlice::from_slice_unchecked(server.collection()) };
                    link_query
                        .iter_many_unique_mut(unique_slice)
                        .for_each(|(entity, mut link)| {
                            let mut entity_mut = c.entity(entity);

                            // #[cfg(feature = "test_utils")]
                            // trace!("SERVER: length of each packet in receive: {:?}", link.recv.iter().map(|p| p.len()).collect::<Vec<_>>());

                            // TODO: insert Connecting if we receive a ConnectionRequest packet
                            match netcode_server.inner.receive(link.as_mut(), &mut entity_mut) {
                                Ok(errors) => {
                                    for error in errors {
                                        error.log();
                                    }
                                }
                                Err(e) => {
                                    error!("Error receiving packet: {:?}", e);
                                }
                            }
                        });

                    // Connections: we know the connection comes from the current entity!
                    netcode_server
                        .inner
                        .cfg
                        .context
                        .connections
                        .drain(..)
                        .for_each(|(id, entity)| {
                            // TODO: mention server id in case we have multiple servers
                            info!("New connection on netcode from {:?} ({:?})", id, entity);
                            trace!("Adding Connected/ClientOf with id {:?}", id);
                            c.entity(entity).insert((
                                Connected,
                                LocalId(PeerId::Server),
                                RemoteId(PeerId::Netcode(id)),
                                ClientOf,
                            ));
                        });
                    netcode_server
                        .inner
                        .cfg
                        .context
                        .disconnections
                        .drain(..)
                        .for_each(|(id, entity)| {
                            // TODO: mention server id in case we have multiple servers
                            info!(
                                "Disconnection from netcode client {:?}. Despawning entity.",
                                id
                            );
                            // first disconnect to trigger observers
                            c.entity(entity)
                                .try_insert(Disconnected { reason: None })
                                .despawn();
                        });
                    if stopping {
                        // after we sent disconnection packets, we can stop the server
                        c.entity(server_entity).insert(Stopped);
                    }
                });
            },
        )
    }

    fn start(
        trigger: Trigger<Start>,
        query: Query<(), With<NetcodeServer>>,
        mut commands: Commands,
    ) {
        if query.get(trigger.target()).is_ok() {
            commands.entity(trigger.target()).insert(Started);
        }
    }

    fn stop(
        trigger: Trigger<Stop>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut NetcodeServer, &Server), Without<Stopped>>,
        mut link_query: Query<
            (Entity, &mut Link, &RemoteId),
            (With<ClientOf>, With<Connected>, Without<HostClient>),
        >,
    ) -> Result {
        if let Ok((server_entity, mut netcode_server, server)) = query.get_mut(trigger.target()) {
            info!("Stopping netcode server");

            // TODO: should we stop the io?
            // // stop the ServerIo that is on this entity (for example webtransport server)
            // commands.trigger_targets(Unlink, server_entity);
            commands.entity(server_entity).insert(Stopping);

            // SAFETY: we know that the list of client entities are unique because it is a Relationship
            let unique_slice =
                unsafe { UniqueEntitySlice::from_slice_unchecked(server.collection()) };
            link_query.iter_many_unique_mut(unique_slice).try_for_each(
                |(entity, mut link, remote_peer_id)| {
                    let PeerId::Netcode(client_id) = remote_peer_id.0 else {
                        error!("Client {:?} is not a Netcode client", remote_peer_id);
                        return Err(crate::error::Error::UnknownClient(remote_peer_id.0));
                    };
                    // this will make sure that `netcode.on_disconnect` is called, so the entity will get disconnected
                    // in the next frame from the `receive` system.
                    netcode_server.inner.disconnect(client_id, &mut link.send)?;
                    commands.entity(entity).insert(Disconnecting);
                    Ok(())
                },
            )?;
        }
        Ok(())
    }
}

impl Plugin for NetcodeServerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<lightyear_connection::client::ConnectionPlugin>() {
            app.add_plugins(lightyear_connection::client::ConnectionPlugin);
        }
        if !app.is_plugin_added::<lightyear_connection::server::ConnectionPlugin>() {
            app.add_plugins(lightyear_connection::server::ConnectionPlugin);
        }
        // TODO: should these be shared? or do we use Markers like in lightyear to distinguish between client and server?
        app.configure_sets(
            PreUpdate,
            (
                LinkSet::Receive,
                ConnectionSet::Receive,
                TransportSet::Receive,
            )
                .chain(),
        );
        app.configure_sets(
            PostUpdate,
            (TransportSet::Send, ConnectionSet::Send, LinkSet::Send).chain(),
        );

        app.add_systems(PreUpdate, Self::receive.in_set(ConnectionSet::Receive));
        app.add_systems(PostUpdate, Self::send.in_set(ConnectionSet::Send));

        app.add_observer(Self::start);
        app.add_observer(Self::stop);
    }
}
