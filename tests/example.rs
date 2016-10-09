extern crate raft;

use raft::*;
use raft::persistent_log::mock::MockPersistentLog;
use raft::state_machine::mock::MockStateMachine;

#[test]
fn creation() {
    let persistent_log = MockPersistentLog::new();
    let state_machine = MockStateMachine::new();

    let raft = Raft::new(persistent_log, state_machine);
}
