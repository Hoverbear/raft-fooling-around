use persistent_log::PersistentLog;
use state_machine::StateMachine;

pub trait Raft {
    type PersistentLog: PersistentLog;
    type StateMachine: StateMachine;
}
