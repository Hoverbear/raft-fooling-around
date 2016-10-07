use raft::Raft;
use persistent_log::PersistentLog;
use state_machine::StateMachine;

use super::candidate::Candidate;

#[derive(Debug)]
pub struct Follower<L, M> where L: PersistentLog, M: StateMachine {
    persistent_log: L,
    state_machine: M,
}

impl<L, M> Follower<L, M> where L: PersistentLog, M: StateMachine {
    pub fn new(persistent_log: L, state_machine: M) -> Self {
        Follower {
            persistent_log: persistent_log,
            state_machine: state_machine,
        }
    }
}

impl<L, M> Into<Candidate<L, M>> for Follower<L, M> where L: PersistentLog, M: StateMachine {
    fn into(self) -> Candidate<L, M> {
        Candidate::new(self.persistent_log, self.state_machine)
    }
}

impl<L, M> Raft for Follower<L, M>  where L: PersistentLog, M: StateMachine {
    type PersistentLog = L;
    type StateMachine = M;
}
