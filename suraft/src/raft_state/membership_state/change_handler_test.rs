use std::sync::Arc;

use maplit::btreemap;
use maplit::btreeset;

use crate::engine::testing::s;
use crate::engine::testing::UTConfig;
use crate::error::ChangeMembershipError;
use crate::error::EmptyMembership;
use crate::error::InProgress;
use crate::error::LearnerNotFound;
use crate::testing::log_id;
use crate::ChangeMembers;
use crate::EffectiveMembership;
use crate::Membership;
use crate::MembershipState;

/// Create an Arc<EffectiveMembership>
fn effmem(term: u64, index: u64, m: Membership<UTConfig>) -> Arc<EffectiveMembership<UTConfig>> {
    let lid = Some(log_id(term, index));
    Arc::new(EffectiveMembership::new(lid, m))
}

fn m1() -> Membership<UTConfig> {
    Membership::new(vec![btreeset! {s(1)}], None)
}

fn m12() -> Membership<UTConfig> {
    Membership::new(vec![btreeset! {s(1),s(2)}], None)
}

fn m123_345() -> Membership<UTConfig> {
    Membership::new(vec![btreeset! {s(1), s(2), s(3)}, btreeset! {s(3), s(4), s(5)}], None)
}

#[test]
fn test_apply_not_committed() -> anyhow::Result<()> {
    let new = || MembershipState::<UTConfig>::new(effmem(2, 2, m1()), effmem(3, 4, m123_345()));
    let res = new().change_handler().apply(ChangeMembers::AddVoterIds(btreeset! {s(1)}), false);

    assert_eq!(
        Err(ChangeMembershipError::InProgress(InProgress {
            committed: Some(log_id(2, 2)),
            membership_log_id: Some(log_id(3, 4))
        })),
        res
    );

    Ok(())
}

#[test]
fn test_apply_empty_voters() -> anyhow::Result<()> {
    let new = || MembershipState::<UTConfig>::new(effmem(3, 4, m1()), effmem(3, 4, m1()));
    let res = new().change_handler().apply(ChangeMembers::RemoveVoters(btreeset! {s(1)}), false);

    assert_eq!(Err(ChangeMembershipError::EmptyMembership(EmptyMembership {})), res);

    Ok(())
}

#[test]
fn test_apply_learner_not_found() -> anyhow::Result<()> {
    let new = || MembershipState::<UTConfig>::new(effmem(3, 4, m1()), effmem(3, 4, m1()));
    let res = new().change_handler().apply(ChangeMembers::AddVoterIds(btreeset! {s(2)}), false);

    assert_eq!(
        Err(ChangeMembershipError::LearnerNotFound(LearnerNotFound {
            node_id: s(2)
        })),
        res
    );

    Ok(())
}

#[test]
fn test_apply_retain_learner() -> anyhow::Result<()> {
    let new = || MembershipState::<UTConfig>::new(effmem(3, 4, m12()), effmem(3, 4, m123_345()));

    // Do not leave removed voters as learner
    let res = new().change_handler().apply(ChangeMembers::RemoveVoters(btreeset! {s(1),s(2)}), false);
    assert_eq!(
        Ok(Membership::new(
            vec![btreeset! {s(3), s(4), s(5)}],
            btreemap! {s(3)=>(),s(4)=>(),s(5)=>()}
        )),
        res
    );

    // Leave removed voters as learner
    let res = new().change_handler().apply(ChangeMembers::RemoveVoters(btreeset! {s(1),s(2)}), true);
    assert_eq!(
        Ok(Membership::new(
            vec![btreeset! {s(3), s(4), s(5)}],
            btreemap! {s(1)=>(),s(2)=>(),s(3)=>(),s(4)=>(),s(5)=>()}
        )),
        res
    );

    Ok(())
}
