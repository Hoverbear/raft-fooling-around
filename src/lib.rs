pub mod state_machine;
pub mod persistent_log;
pub mod states;
pub mod raft;

pub use state_machine::StateMachine;
pub use persistent_log::PersistentLog;
pub use raft::Raft;
pub use states::{Leader, Candidate, Follower};

#[cfg(test)]
mod test_transitions {
    use super::*;

    #[derive(Debug)]
    struct MockPersistentLog;
    impl PersistentLog for MockPersistentLog {}

    #[derive(Debug)]
    struct MockStateMachine;
    impl StateMachine for MockStateMachine {}

    #[test]
    fn test_transitions() {
        let (persistent_log, state_machine) = (MockPersistentLog, MockStateMachine);

        let leader = Leader::new(persistent_log, state_machine);
        let follower: Follower<_, _> = leader.into();
        let candidate: Candidate<_, _> = follower.into();
        let _: Leader<_, _> = candidate.into();
    }
}
