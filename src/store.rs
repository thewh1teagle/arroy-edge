use std::path::PathBuf;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use anyhow::Result;

pub struct Store {
    db: PickleDb
}

impl Store {
    pub fn try_create(path: &str) -> Result<Self> {
        let db: PickleDb;
        match PathBuf::from(path).exists() {
            false => {
                log::debug!("DB file not found. creating new one");
                db = PickleDb::new(path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
            },
            _ => {
                log::debug!("Loading existing DB");
                db = PickleDb::load(path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?;
            }
        }

        Ok( Self { db } )
    }
    pub fn set(&mut self, id: &str, value: &str) -> Result<()> {
        self.db.set(id, &value)?;
        Ok(())
    }

    pub fn get(&mut self, id: String) -> Result<Option<String>> {
        let resp = self.db.get(&id);
        Ok(resp)
    }

    pub fn remove(&mut self, id: String) -> Result<()> {
        self.db.rem(&id)?;
        Ok(())
    }

    pub fn all(&mut self) -> Result<Vec<(String, Option<String>)>> {
        let resp: Vec<(String, Option<String>)> = self.db.iter().map(|i| (i.get_key().to_owned(), i.get_value::<String>().to_owned())).collect();
        Ok(resp)
    }
}