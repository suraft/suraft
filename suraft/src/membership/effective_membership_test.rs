use maplit::btreeset;

use crate::engine::testing::s;
use crate::engine::testing::UTConfig;
use crate::quorum::QuorumSet;
use crate::EffectiveMembership;
use crate::Membership;

#[test]
fn test_effective_membership_majority() -> anyhow::Result<()> {
    {
        let m12345 = Membership::<UTConfig>::new(vec![btreeset! {s(1),s(2),s(3),s(4),s(5) }], None);
        let m = EffectiveMembership::<UTConfig>::new(None, m12345);

        assert!(!m.is_quorum([s(0)].iter()));
        assert!(!m.is_quorum([s(0), s(1), s(2)].iter()));
        assert!(!m.is_quorum([s(6), s(7), s(8)].iter()));
        assert!(m.is_quorum([s(1), s(2), s(3)].iter()));
        assert!(m.is_quorum([s(3), s(4), s(5)].iter()));
        assert!(m.is_quorum([s(1), s(3), s(4), s(5)].iter()));
    }

    {
        let m12345_678 = Membership::<UTConfig>::new(
            vec![btreeset! {s(1),s(2),s(3),s(4),s(5) }, btreeset! {s(6),s(7),s(8)}],
            None,
        );
        let m = EffectiveMembership::<UTConfig>::new(None, m12345_678);

        assert!(!m.is_quorum([s(0)].iter()));
        assert!(!m.is_quorum([s(0), s(1), s(2)].iter()));
        assert!(!m.is_quorum([s(6), s(7), s(8)].iter()));
        assert!(!m.is_quorum([s(1), s(2), s(3)].iter()));
        assert!(m.is_quorum([s(1), s(2), s(3), s(6), s(7)].iter()));
        assert!(m.is_quorum([s(1), s(2), s(3), s(4), s(7), s(8)].iter()));
    }

    Ok(())
}
