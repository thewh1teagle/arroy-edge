use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use anyhow::Result;

pub struct Store {
    db: PickleDb
}

impl Store {
    pub fn new(path: &str) -> Self {
        let db = PickleDb::new(path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
        Self { db }
    }
    pub fn set(&mut self, id: &str, value: &str) -> Result<()> {
        self.db.set(id, &value)?;
        Ok(())
    }

    pub fn get(&mut self, id: String) -> Result<Option<String>> {
        let resp = self.db.get(&id);
        Ok(resp)
    }
}