use crate::client_server::prediction::{
    RollbackInfo, trigger_rollback, trigger_rollback_check, trigger_rollback_system,
};
use crate::protocol::{CompFull, CompMap, CompNotNetworked};
use crate::stepper::ClientServerStepper;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use bevy::prelude::*;
use core::time::Duration;
use lightyear::prediction::Predicted;
use lightyear::prediction::diagnostics::PredictionMetrics;
use lightyear::prediction::predicted_history::PredictionHistory;
use lightyear_connection::prelude::NetworkTarget;
use lightyear_messages::MessageManager;
use lightyear_prediction::prelude::{PredictionManager, RollbackSet};
use lightyear_prediction::rollback::DeterministicPredicted;
use lightyear_replication::components::Confirmed;
use lightyear_replication::prelude::{PredictionTarget, Replicate, ReplicationSet};
use test_log::test;

fn setup() -> (ClientServerStepper, Entity, Entity) {
    let mut stepper = ClientServerStepper::single();
    stepper.client_app().add_event::<RollbackInfo>();
    stepper.client_app().add_systems(
        PreUpdate,
        trigger_rollback_system
            .after(ReplicationSet::Receive)
            .before(RollbackSet::Check),
    );

    // add predicted/confirmed entities
    let tick = stepper.client_tick(0);
    let predicted = stepper
        .client_app()
        .world_mut()
        .spawn(Predicted {
            confirmed_entity: None,
        })
        .id();
    let confirmed = stepper
        .client_app()
        .world_mut()
        .spawn((
            Confirmed {
                tick,
                predicted: Some(predicted),
                ..Default::default()
            },
            CompFull(1.0),
        ))
        .id();
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .get_mut::<Predicted>()
        .unwrap()
        .confirmed_entity = Some(confirmed);
    (stepper, confirmed, predicted)
}

struct RollbackCounter(pub usize);

// TODO: check that if A is updated but B is not, and A and B are in the same replication group,
//  then we need to check the rollback for B as well!
/// Check that we enter a rollback state when confirmed entity is updated at tick T and:
/// 1. Predicted component and Confirmed component are different
/// 2. Confirmed component does not exist and predicted component exists
/// 3. Confirmed component exists but predicted component does not exist
/// 4. If confirmed component is the same value as what we have in the history for predicted component, we do not rollback
#[test]
fn test_check_rollback() {
    let (mut stepper, confirmed, predicted) = setup();

    // make sure we simulate that we received a server update
    let tick = stepper.client_tick(0);

    // step once to avoid 0 tick rollback
    stepper.frame_step(1);

    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);
    // 0. Rollback when the Confirmed component is just added
    // (there is a rollback even though the values match, because the value isn't present in
    //  the PredictionHistory at the time of spawn)
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        1
    );

    // 1. Predicted component and confirmed component are different
    let tick = stepper.client_tick(0);
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed)
        .insert(CompFull(2.0));
    // simulate that we received a server message for the confirmed entity on tick `tick`
    // where the PredictionHistory had the value of 1.0

    // step once to avoid 0 tick rollback
    stepper.frame_step(1);

    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        2
    );
    // the predicted history now has CompFull(2.0)

    // 2. Confirmed component does not exist but predicted component exists
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed)
        .remove::<CompFull>();
    // simulate that we received a server message for the confirmed entity on tick `tick`
    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        3
    );
    // the predicted history now has Absent

    // 3. Confirmed component exists but predicted component does not exist
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .remove::<CompFull>();
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed)
        .insert(CompFull(2.0));
    // simulate that we received a server message for the confirmed entity on tick `tick`
    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        4
    );
    // the predicted history now has ConfirmedSyncModeFull(2.0)

    // 4. If confirmed component is the same value as what we have in the history for predicted component, we do not rollback
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .get_mut::<PredictionHistory<CompFull>>()
        .unwrap()
        .add_update(tick, CompFull(2.0));

    // simulate that we received a server message for the confirmed entity on tick `tick`
    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);
    // no rollback
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        4
    );
}

/// Test that the entities within a predicted component marked as to be
/// entity-mapped are mapped when rollbacked.
#[test]
fn test_rollback_entity_mapping() {
    let mut stepper = ClientServerStepper::single();

    // Spawn a remote entity with a `CompMap` component that
    // points to the remote entity. This entity will be replicated to the
    // client and predicted by the client.
    let remote_entity = stepper.server_app.world_mut().spawn_empty().id();
    stepper
        .server_app
        .world_mut()
        .entity_mut(remote_entity)
        .insert((
            CompMap(remote_entity),
            Replicate::to_clients(NetworkTarget::All),
            PredictionTarget::to_clients(NetworkTarget::All),
        ));

    stepper.frame_step(2);

    // Get the confirmed and predicted entities associated with `remote_entity`.
    let confirmed_entity = stepper
        .client(0)
        .get::<MessageManager>()
        .unwrap()
        .entity_mapper
        .get_local(remote_entity)
        .expect("entity is not present in entity map");
    let predicted_entity = *stepper
        .client_mut(0)
        .get_mut::<PredictionManager>()
        .unwrap()
        .predicted_entity_map
        .get_mut()
        .confirmed_to_predicted
        .get(&confirmed_entity)
        .unwrap();

    // Modify `predicted_entity`'s `CompMap` to point to some
    // incorrect value, perform a rollback, and verify that
    // `predicted_entity`'s `ComponentWithEntity` points to
    // `predicted_entity`. (and not `confirmed_entity`)
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted_entity)
        .get_mut::<CompMap>()
        .unwrap()
        .0 = Entity::PLACEHOLDER;
    let tick = stepper.client_tick(0);
    trigger_rollback(&mut stepper, tick);
    stepper.frame_step(1);
    assert_eq!(
        stepper
            .client_app()
            .world()
            .entity(predicted_entity)
            .get::<CompMap>()
            .unwrap()
            .0,
        predicted_entity,
        "Expected predicted component to point to predicted entity"
    );

    // Delete `predicted_entity`'s `ComponentWithEntity`, perform a
    // rollback, and verify that `predicted_entity`'s
    // `ComponentWithEntity` gets re-created and points to
    // `predicted_entity`.
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted_entity)
        .remove::<CompMap>();
    let tick = stepper.client_tick(0);
    trigger_rollback(&mut stepper, tick);
    stepper.frame_step(1);
    assert_eq!(
        stepper
            .client_app()
            .world_mut()
            .entity_mut(predicted_entity)
            .get_mut::<CompMap>()
            .unwrap()
            .0,
        predicted_entity,
        "Expected predicted component to point to predicted entity"
    );
}

/// Test that:
/// - we remove a component from the predicted entity
/// - rolling back before the remove should re-add it
///   We are still able to rollback properly (the rollback adds the component to the predicted entity)
#[test]
fn test_removed_predicted_component_rollback() {
    let (mut stepper, confirmed, predicted) = setup();
    fn increment_component_system(
        mut commands: Commands,
        mut query_networked: Query<(Entity, &mut CompFull), With<Predicted>>,
    ) {
        for (entity, mut component) in query_networked.iter_mut() {
            component.0 += 1.0;
            if component.0 == 5.0 {
                commands.entity(entity).remove::<CompFull>();
            }
        }
    }
    stepper
        .client_app()
        .add_systems(FixedUpdate, increment_component_system);
    stepper.frame_step(1);

    // check that the component got synced
    assert_eq!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted)
            .unwrap(),
        &CompFull(2.0)
    );
    // also insert a non-networked component directly on the predicted entity
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .insert(CompNotNetworked(1.0));

    // advance five more frames, so that the component gets removed on predicted
    stepper.frame_step(5);

    // check that the networked component got removed on predicted
    assert!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted)
            .is_none()
    );
    // also remove the non-networked component
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .remove::<CompNotNetworked>();

    // create a rollback situation
    let tick = stepper.client_tick(0);
    info!("Trigger rollback back to {:?}", tick - 3);
    stepper
        .client_app()
        .world_mut()
        .get_mut::<CompFull>(confirmed)
        .unwrap()
        .0 = -10.0;
    trigger_rollback_check(&mut stepper, tick - 3);
    stepper.frame_step(1);

    // check that rollback happened
    // predicted got the component re-added and that we rolled back 3 ticks and advances by 1 tick
    assert_eq!(
        stepper
            .client_app()
            .world_mut()
            .get_mut::<CompFull>(predicted)
            .unwrap()
            .0,
        -6.0
    );
    // the non-networked component got rolled back as well
    assert_eq!(
        stepper
            .client_app()
            .world_mut()
            .get_mut::<CompNotNetworked>(predicted)
            .unwrap()
            .0,
        1.0
    );
}

/// Test that:
/// - a component gets added on Predicted
/// - we trigger a rollback, and the confirmed entity does not have the component
/// - the rollback removes the component from the predicted entity
#[test]
fn test_added_predicted_component_rollback() {
    let (mut stepper, confirmed, predicted) = setup();

    stepper.frame_step(1);

    // the prediction history is updated with this tick
    let rollback_tick = stepper.client_tick(0);
    stepper.frame_step(1);

    // add a non-networked component as well, which should be removed on the rollback
    // since it did not exist at the rollback tick
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted)
        .insert(CompNotNetworked(1.0));

    // create a rollback situation to a tick where
    // - confirmed_component missing
    // - predicted component exists in history
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed)
        .remove::<CompFull>();
    trigger_rollback_check(&mut stepper, rollback_tick);
    stepper.frame_step(1);

    // check that rollback happened:
    // the registered component got removed from predicted since it was not present on confirmed
    assert!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted)
            .is_none()
    );
    // the non-networked component got removed from predicted as it wasn't present in the history
    assert!(
        stepper
            .client_app()
            .world()
            .get::<CompNotNetworked>(predicted)
            .is_none()
    );
}

/// If we have disable_rollback:
/// 1) we don't check rollback for that entity
/// 2) if a rollback happens, we reset to the predicted history value instead of the confirmed value
#[test]
fn test_disable_rollback() {
    let (mut stepper, confirmed_b, predicted_b) = setup();

    // add predicted/confirmed entities
    let tick = stepper.client_tick(0);
    let predicted_a = stepper
        .client_app()
        .world_mut()
        .spawn((
            Predicted {
                confirmed_entity: None,
            },
            DeterministicPredicted,
        ))
        .id();
    let confirmed_a = stepper
        .client_app()
        .world_mut()
        .spawn((
            Confirmed {
                tick,
                predicted: Some(predicted_a),
                ..Default::default()
            },
            CompFull(1.0),
        ))
        .id();
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted_a)
        .get_mut::<Predicted>()
        .unwrap()
        .confirmed_entity = Some(confirmed_a);

    // value gets synced and added to PredictionHistory
    stepper.frame_step(1);

    // 1. check rollback doesn't trigger on disable-rollback entities
    let tick = stepper.client_tick(0);
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed_a)
        .get_mut::<CompFull>()
        .unwrap()
        .0 = 2.0;
    // simulate that we received a server message for the confirmed entity on tick `tick`
    trigger_rollback_check(&mut stepper, tick);
    let num_rollbacks = stepper
        .client_app()
        .world()
        .resource::<PredictionMetrics>()
        .rollbacks;
    stepper.frame_step(1);
    // no rollback
    assert_eq!(
        stepper
            .client_app()
            .world()
            .resource::<PredictionMetrics>()
            .rollbacks,
        num_rollbacks
    );

    // 2. If a rollback happens, then we reset DisableRollback entities to their historical value
    stepper.frame_step(1);
    let tick = stepper.client_tick(0);
    stepper
        .client_app()
        .world_mut()
        .entity_mut(confirmed_b)
        .get_mut::<CompFull>()
        .unwrap()
        .0 = 3.0;
    let mut history = PredictionHistory::<CompFull>::default();
    history.add_update(tick, CompFull(10.0));
    stepper
        .client_app()
        .world_mut()
        .entity_mut(predicted_a)
        .insert(history);
    // step once to avoid a 0-tick rollback
    stepper.frame_step(1);
    // simulate that we received a server message for the confirmed entities on tick `tick`
    // (all predicted entities are in the same ReplicationGroup)
    trigger_rollback_check(&mut stepper, tick);
    stepper.frame_step(1);

    // the DisableRollback entity was rolledback to the past PredictionHistory value
    assert_eq!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted_a)
            .unwrap()
            .0,
        10.0
    );
    assert_eq!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted_b)
            .unwrap()
            .0,
        3.0
    );
}

/// Test that:
/// - the `Time` resource's elapsed is rollbacked to the first tick of the rollback
/// - the `Time` resource's elapsed time is advanced correctly during the rollback
/// - the `Time` resource's delta during a rollback is the `Time<Fixed>`'s delta
#[test]
fn test_rollback_time_resource() {
    #[derive(Debug, PartialEq)]
    struct TimeSnapshot {
        is_rollback: bool,
        delta: Duration,
        elapsed: Duration,
    }

    #[derive(Resource, Default, Debug)]
    struct TimeTracker {
        snapshots: Vec<TimeSnapshot>,
    }

    // Record the time resource's values for each tick.
    fn track_time(
        time: Res<Time>,
        mut time_tracker: ResMut<TimeTracker>,
        rollback: Single<&PredictionManager>,
    ) {
        time_tracker.snapshots.push(TimeSnapshot {
            is_rollback: rollback.is_rollback(),
            delta: time.delta(),
            elapsed: time.elapsed(),
        });
    }

    let (mut stepper, confirmed, predicted) = setup();
    stepper.client_app().insert_resource(TimeTracker::default());
    stepper.client_app().add_systems(FixedUpdate, track_time);
    let time_before_next_tick = *stepper.client_app().world().resource::<Time<Fixed>>();

    // Trigger 2 rollback ticks
    let tick = stepper.client_tick(0);
    trigger_rollback_check(&mut stepper, tick - 2);
    stepper.frame_step(1);

    // Check that the component got synced.
    assert_eq!(
        stepper
            .client_app()
            .world()
            .get::<CompFull>(predicted)
            .unwrap(),
        &CompFull(1.0)
    );

    // Verify that the 2 rollback ticks and regular tick occurred with the
    // correct delta times and elapsed times.
    let tick_duration = stepper.tick_duration;
    let time_tracker = stepper.client_app().world().resource::<TimeTracker>();
    assert_eq!(
        time_tracker.snapshots,
        vec![
            TimeSnapshot {
                is_rollback: true,
                delta: tick_duration,
                elapsed: time_before_next_tick.elapsed() - tick_duration
            },
            TimeSnapshot {
                is_rollback: true,
                delta: tick_duration,
                elapsed: time_before_next_tick.elapsed()
            },
            TimeSnapshot {
                is_rollback: false,
                delta: tick_duration,
                elapsed: time_before_next_tick.elapsed() + tick_duration
            }
        ]
    );
}
