use super::PersistentLog;

pub struct MockPersistentLog;
impl MockPersistentLog {
    pub fn new() -> Self {
        MockPersistentLog
    }
}
impl PersistentLog for MockPersistentLog {}
