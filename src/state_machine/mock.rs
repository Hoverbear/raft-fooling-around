use super::StateMachine;

pub struct MockStateMachine;
impl MockStateMachine {
    pub fn new() -> Self {
        MockStateMachine
    }
}
impl StateMachine for MockStateMachine {}
