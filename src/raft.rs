use persistent_log::PersistentLog;
use state_machine::StateMachine;

pub struct Raft<L, M, S>
    where L: PersistentLog, M: StateMachine
{
    persistent_log: L,
    state_machine: M,
    state: ::std::marker::PhantomData<S>,
}

impl<L, M> Raft<L, M, Candidate>
    where L: PersistentLog, M: StateMachine
{
    pub fn new(log: L, machine: M) -> Self {
        Raft {
            persistent_log: log,
            state_machine: machine,
            state: ::std::marker::PhantomData,
        }
    }
}

macro_rules! raft_states {
    // hardcoded for a single #[doc = "oneline documentation"] for simplicity
    ($(#[$attr:meta] $from:ident -> $($to:ident)|*),* $(,)*) => { $(
        #[$attr] pub enum $from {}
        $(
            impl<L, M> Into<Raft<L, M, $to>> for Raft<L, M, $from>
                where L: PersistentLog, M: StateMachine
            {
                fn into(self) -> Raft<L, M, $to> {
                    Raft {
                        persistent_log: self.persistent_log,
                        state_machine: self.state_machine,
                        state: ::std::marker::PhantomData,
                    }
                }
            }
        )*
    )*}
}

raft_states!{
    /// Candidate: may become either a follower or a leader
    Candidate -> Follower | Leader,
    /// Follower: is currently following a leader
    Follower  -> Candidate,
    /// Leader: is the current leader
    Leader    -> Candidate,
}

pub enum RaftState<L, M>
    where L: PersistentLog, M: StateMachine
{
    Candidate(Raft<L, M, Candidate>),
    Follower(Raft<L, M, Follower>),
    Leader(Raft<L, M, Leader>),
}

#[cfg(test)]
mod test {
    use persistent_log::mock::MockPersistentLog;
    use state_machine::mock::MockStateMachine;
    use {Raft, Candidate, Follower, Leader, RaftState};

    #[test]
    fn transitions_stack() {
        let persistent_log = MockPersistentLog::default();
        let state_machine = MockStateMachine::default();
        let raft: Raft<_, _, Candidate> = Raft::new(persistent_log, state_machine);
        let raft: Raft<_, _, Follower> = raft.into();
        let raft: Raft<_, _, Candidate> = raft.into();
        let _raft: Raft<_, _, Leader> = raft.into();
    }

    #[test]
    fn transitions_enum() {
        macro_rules! assert_trans {
            ($query:expr, $pat:pat => $expr:expr) => {
                match $query {
                    $pat => $expr,
                    _ => unreachable!()
                }
            }
        }
        let persistent_log = MockPersistentLog::default();
        let state_machine = MockStateMachine::default();
        let mut state = RaftState::Candidate(Raft::new(persistent_log, state_machine));
        state = assert_trans!(state, RaftState::Candidate(r) => RaftState::Follower(r.into()));
        state = assert_trans!(state, RaftState::Follower(r) => RaftState::Candidate(r.into()));
        state = assert_trans!(state, RaftState::Candidate(r) => RaftState::Leader(r.into()));
        // assert is now Leader
        assert_trans!(state, RaftState::Leader(_) => ());
    }
}
