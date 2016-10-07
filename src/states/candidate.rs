use raft::Raft;
use persistent_log::PersistentLog;
use state_machine::StateMachine;

use super::follower::Follower;
use super::leader::Leader;

#[derive(Debug)]
pub struct Candidate<L, M> where L: PersistentLog, M: StateMachine {
    persistent_log: L,
    state_machine: M,
}

impl<L, M> Candidate<L, M> where L: PersistentLog, M: StateMachine {
    pub fn new(persistent_log: L, state_machine: M) -> Self {
        Candidate {
            persistent_log: persistent_log,
            state_machine: state_machine,
        }
    }
}

impl<L, M> Into<Leader<L, M>> for Candidate<L, M> where L: PersistentLog, M: StateMachine {
    fn into(self) -> Leader<L, M> {
        Leader::new(self.persistent_log, self.state_machine)
    }
}

impl<L, M> Into<Follower<L, M>> for Candidate<L, M> where L: PersistentLog, M: StateMachine {
    fn into(self) -> Follower<L, M> {
        Follower::new(self.persistent_log, self.state_machine)
    }
}

impl<L, M> Raft for Candidate<L, M>  where L: PersistentLog, M: StateMachine {
    type PersistentLog = L;
    type StateMachine = M;
}
