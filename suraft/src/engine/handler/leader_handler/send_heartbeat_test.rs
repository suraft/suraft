use std::sync::Arc;
use std::time::Duration;

use maplit::btreeset;
use pretty_assertions::assert_eq;

use crate::engine::testing::s;
use crate::engine::testing::UTConfig;
use crate::engine::Command;
use crate::engine::Engine;
use crate::replication::ReplicationSessionId;
use crate::testing::log_id;
use crate::type_config::TypeConfigExt;
use crate::utime::Leased;
use crate::EffectiveMembership;
use crate::Membership;
use crate::MembershipState;
use crate::Vote;

fn m01() -> Membership {
    Membership::new(vec![btreeset! {s(0),s(1)}], None)
}

fn m23() -> Membership {
    Membership::new(vec![btreeset! {s(2), s(3)}], btreeset! {s(1), s(2), s(3)})
}

fn eng() -> Engine<UTConfig> {
    let mut eng = Engine::testing_default(s(0));
    eng.state.enable_validation(false); // Disable validation for incomplete state

    eng.config.id = s(1);
    eng.state.committed = Some(log_id(0, 0));
    eng.state.vote = Leased::new(
        UTConfig::now(),
        Duration::from_millis(500),
        Vote::new_committed(3, s(1)),
    );
    eng.state.log_ids.append(log_id(1, 1));
    eng.state.log_ids.append(log_id(2, 3));
    eng.state.membership_state = MembershipState::new(
        Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m01())),
        Arc::new(EffectiveMembership::new(Some(log_id(2, 3)), m23())),
    );
    eng.testing_new_leader();
    eng.state.server_state = eng.calc_server_state();

    eng
}

#[test]
fn test_leader_send_heartbeat() -> anyhow::Result<()> {
    let mut eng = eng();
    eng.output.take_commands();

    // A heartbeat is a normal AppendEntries RPC if there are pending data to send.
    {
        eng.leader_handler()?.send_heartbeat();
        assert_eq!(
            vec![
                //
                Command::BroadcastHeartbeat {
                    session_id: ReplicationSessionId::new(Vote::new(3, s(1)).into_committed(), Some(log_id(2, 3))),
                    committed: Some(log_id(0, 0))
                },
            ],
            eng.output.take_commands()
        );
    }

    // Heartbeat will be resent
    {
        eng.output.clear_commands();
        eng.leader_handler()?.send_heartbeat();
        assert_eq!(
            vec![
                //
                Command::BroadcastHeartbeat {
                    session_id: ReplicationSessionId::new(Vote::new(3, s(1)).into_committed(), Some(log_id(2, 3))),
                    committed: Some(log_id(0, 0))
                },
            ],
            eng.output.take_commands()
        );
    }

    Ok(())
}
