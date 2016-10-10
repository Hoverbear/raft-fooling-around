extern crate raft;

use raft::*;
use raft::persistent_log::mock::MockPersistentLog;
use raft::state_machine::mock::MockStateMachine;

#[test]
fn creation() {
    let persistent_log = MockPersistentLog::default();
    let state_machine = MockStateMachine::default();

    let _raft = Raft::new(persistent_log, state_machine);
}
