use raft::Raft;
use persistent_log::PersistentLog;
use state_machine::StateMachine;

use super::follower::Follower;

#[derive(Debug)]
pub struct Leader<L, M> where L: PersistentLog, M: StateMachine {
    persistent_log: L,
    state_machine: M,
}

impl<L, M> Leader<L, M> where L: PersistentLog, M: StateMachine {
    pub fn new(persistent_log: L, state_machine: M) -> Self {
        Leader {
            persistent_log: persistent_log,
            state_machine: state_machine,
        }
    }
}

impl<L, M> Into<Follower<L, M>> for Leader<L, M> where L: PersistentLog, M: StateMachine {
    fn into(self) -> Follower<L, M> {
        Follower::new(self.persistent_log, self.state_machine)
    }
}

impl<L, M> Raft for Leader<L, M>  where L: PersistentLog, M: StateMachine {
    type PersistentLog = L;
    type StateMachine = M;
}
