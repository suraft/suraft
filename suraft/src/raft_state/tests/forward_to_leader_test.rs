use std::sync::Arc;
use std::time::Duration;

use maplit::btreemap;
use maplit::btreeset;

use crate::engine::testing::s;
use crate::engine::testing::UTConfig;
use crate::error::ForwardToLeader;
use crate::type_config::TypeConfigExt;
use crate::utime::Leased;
use crate::EffectiveMembership;
use crate::LogId;
use crate::Membership;
use crate::MembershipState;
use crate::RaftState;
use crate::Vote;

fn log_id(term: u64, index: u64) -> LogId {
    LogId { term, index }
}

fn m12() -> Membership {
    Membership::new(vec![btreeset! {s(1),s(2)}], None)
}

#[test]
fn test_forward_to_leader_vote_not_committed() {
    let rs = RaftState::<UTConfig> {
        vote: Leased::new(UTConfig::now(), Duration::from_millis(500), Vote::new(1, s(2))),
        membership_state: MembershipState::new(
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m12())),
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m12())),
        ),
        ..Default::default()
    };

    assert_eq!(ForwardToLeader::empty(), rs.forward_to_leader());
}

#[test]
fn test_forward_to_leader_not_a_member() {
    let rs = RaftState::<UTConfig> {
        vote: Leased::new(
            UTConfig::now(),
            Duration::from_millis(500),
            Vote::new_committed(1, s(3)),
        ),
        membership_state: MembershipState::new(
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m12())),
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m12())),
        ),
        ..Default::default()
    };

    assert_eq!(ForwardToLeader::empty(), rs.forward_to_leader());
}

#[test]
fn test_forward_to_leader_has_leader() {
    let m123 = || {
        Membership::new(
            vec![btreeset! {s(1),s(2)}],
            btreemap! {s(1)=>crate::nn(4),s(2)=>crate::nn(5),s(3)=>crate::nn(6)},
        )
    };

    let rs = RaftState::<UTConfig> {
        vote: Leased::new(
            UTConfig::now(),
            Duration::from_millis(500),
            Vote::new_committed(1, s(3)),
        ),
        membership_state: MembershipState::new(
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m123())),
            Arc::new(EffectiveMembership::new(Some(log_id(1, 1)), m123())),
        ),
        ..Default::default()
    };

    assert_eq!(ForwardToLeader::new(s(3), crate::nn(6)), rs.forward_to_leader());
}
