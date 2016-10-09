use persistent_log::PersistentLog;
use state_machine::StateMachine;

pub struct Raft<L, M, S>
where L: PersistentLog, M: StateMachine {
    persistent_log: L,
    state_machine: M,
    state: S,
}

// Follower -> Candidate
struct Follower;
// Candidate -> (Follower|Leader)
struct Candidate;
// Leader -> Follower
struct Leader;

impl<L, M> Raft<L, M, Candidate>
where L: PersistentLog, M: StateMachine {
    pub fn new(log: L, machine: M) -> Self {
        Raft {
            persistent_log: log,
            state_machine: machine,
            state: Candidate,
        }
    }
    #[cfg(test)]
    pub fn to_follower(self) -> Raft<L, M, Follower> {
        self.into()
    }
}

impl<L, M> Into<Raft<L, M, Follower>> for Raft<L, M, Candidate>
where L: PersistentLog, M: StateMachine {
    fn into(self) -> Raft<L, M, Follower> {
        Raft {
            persistent_log: self.persistent_log,
            state_machine: self.state_machine,
            state: Follower,
        }
    }
}

impl<L, M> Into<Raft<L, M, Leader>> for Raft<L, M, Candidate>
where L: PersistentLog, M: StateMachine {
    fn into(self) -> Raft<L, M, Leader> {
        Raft {
            persistent_log: self.persistent_log,
            state_machine: self.state_machine,
            state: Leader,
        }
    }
}

impl<L, M> Into<Raft<L, M, Candidate>> for Raft<L, M, Follower>
where L: PersistentLog, M: StateMachine {
    fn into(self) -> Raft<L, M, Candidate> {
        Raft {
            persistent_log: self.persistent_log,
            state_machine: self.state_machine,
            state: Candidate,
        }
    }
}

#[cfg(test)]
mod test {
    use persistent_log::mock::MockPersistentLog;
    use state_machine::mock::MockStateMachine;
    use super::{Raft, Follower, Leader, Candidate};

    fn transitions() {
        let persistent_log = MockPersistentLog::new();
        let state_machine = MockStateMachine::new();
        let raft: Raft<MockPersistentLog, MockStateMachine, Candidate> = Raft::new(persistent_log, state_machine);
        let raft: Raft<MockPersistentLog, MockStateMachine, Follower> = raft.into();
        let raft: Raft<MockPersistentLog, MockStateMachine, Candidate> = raft.into();
        let raft: Raft<MockPersistentLog, MockStateMachine, Leader> = raft.into();

    }
}
