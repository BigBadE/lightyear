/*!
Provides a [`ProtocolMessage`] enum that is a wrapper around all the possible messages that can be sent over the network
*/
use crate::_reexport::{InputMessage, ShouldBeInterpolated, ShouldBePredicted};
use crate::connection::events::ConnectionEvents;
use crate::prelude::{EntityMap, MapEntities};
use crate::protocol::channel::ChannelKind;
use crate::protocol::Protocol;
use crate::shared::ping::message::SyncMessage;
use crate::shared::replication::ReplicationMessage;
use crate::shared::time_manager::TimeManager;
use crate::utils::named::Named;
use serde::{Deserialize, Serialize};

// pub enum InternalMessage<P: Protocol> {
//     InputMessage(InputMessage<P::Input>),
// }
//
// pub enum InternalReplication<P: Protocol> {
//     ShouldBePredicted(ShouldBePredicted),
//     ShouldBeInterpolated(ShouldBeInterpolated),
// }

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize, Clone)]
pub enum ProtocolMessage<P: Protocol> {
    Message(P::Message),
    Replication(ReplicationMessage<P::Components, P::ComponentKinds>),
    // the reason why we include sync here instead of doing another MessageManager is so that
    // the sync messages can be added to packets that have other messages
    Sync(SyncMessage),
}

impl<P: Protocol> MapEntities for ProtocolMessage<P> {
    fn map_entities(&mut self, entity_map: &EntityMap) {
        match self {
            ProtocolMessage::Message(x) => {
                x.map_entities(entity_map);
            }
            ProtocolMessage::Replication(x) => {
                x.map_entities(entity_map);
            }
            ProtocolMessage::Sync(x) => {
                x.map_entities(entity_map);
            }
        }
    }
}

impl<P: Protocol> ProtocolMessage<P> {
    pub(crate) fn push_to_events(
        self,
        channel_kind: ChannelKind,
        events: &mut ConnectionEvents<P>,
        entity_map: &EntityMap,
        time_manager: &TimeManager,
    ) {
        match self {
            ProtocolMessage::Message(message) => {
                #[cfg(feature = "metrics")]
                {
                    let message_name = message.name();
                    metrics::increment_counter!(format!("receive_message.{}", message_name));
                }
                events.push_message(channel_kind, message);
            }
            ProtocolMessage::Replication(replication) => match replication {
                ReplicationMessage::SpawnEntity(entity, components) => {
                    // convert the remote entity to the local before sending to events
                    // if we can't find the local entity, just use the remote
                    let local_entity = *entity_map.get_local(entity).unwrap_or(&entity);
                    events.push_spawn(local_entity);
                    for component in components {
                        events.push_insert_component(local_entity, (&component).into());
                    }
                }
                ReplicationMessage::DespawnEntity(entity) => {
                    let local_entity = *entity_map.get_local(entity).unwrap_or(&entity);
                    events.push_despawn(local_entity);
                }
                ReplicationMessage::InsertComponent(entity, component) => {
                    let local_entity = *entity_map.get_local(entity).unwrap_or(&entity);
                    events.push_insert_component(local_entity, (&component).into());
                }
                ReplicationMessage::RemoveComponent(entity, component_kind) => {
                    let local_entity = *entity_map.get_local(entity).unwrap_or(&entity);
                    events.push_remove_component(local_entity, component_kind);
                }
                ReplicationMessage::EntityUpdate(entity, components) => {
                    let local_entity = *entity_map.get_local(entity).unwrap_or(&entity);
                    for component in components {
                        events.push_update_component(local_entity, (&component).into());
                    }
                }
            },
            _ => {}
        }
    }
}