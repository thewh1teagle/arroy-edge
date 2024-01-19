use anyhow::{Result, Ok};
use heed::{EnvOpenOptions, Env};
use arroy::{Database, Reader, Writer, ItemId};
use arroy::distances::DotProduct;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::sync::Arc;

pub struct VectorStore {
    db: Database<DotProduct>,
    env: Arc<Env>,
    dimension: usize
}

impl VectorStore {
    pub fn try_create(path: &str, dimension: usize) -> Result<Self> {
        if !PathBuf::from(path).exists() {
            std::fs::create_dir_all(path)?;
        }
        let env = Arc::new(EnvOpenOptions::new()
            .map_size(1024 * 1024 * 1024 * 2) // 2GiB
            .open(path)?);
        
        // if the path not found, create, otherwise use it
        let env_clone = env.clone();
        let mut wtxn = env_clone.write_txn()?;
        let db: Database<DotProduct> = env.create_database(&mut wtxn.borrow_mut(), None)?;

        Ok(VectorStore {
            db,
            env,
            dimension
        })
    }

    pub fn insert(&self, id: u32, vector: &[f32]) -> Result<()> {
        log::debug!("create wtxn");
        let mut wtxn = self.env.write_txn()?;
        log::debug!("create writer");
        let writer = Writer::<DotProduct>::new(self.db, 0, self.dimension)?;
        log::debug!("add item");
        writer.add_item(&mut wtxn, ItemId::from(id), vector)?;
        log::debug!("create seed");
        let mut rng = StdRng::seed_from_u64(0);
        log::debug!("build");
        writer.build(&mut wtxn, &mut rng, None)?;
        log::debug!("commit");
        wtxn.commit()?;
        Ok(())
    }

    pub fn update(&self, id: u32, vector: &[f32]) -> Result<()> {
        let mut wtxn = self.env.write_txn()?;
        let writer = Writer::<DotProduct>::new(self.db, 0, self.dimension)?;
        self.remove(id)?;
        self.insert(id, vector)?;
        Ok(())
    }

    pub fn remove(&self, id: u32) -> Result<()> {
        let mut wtxn = self.env.write_txn()?;
        let writer = Writer::<DotProduct>::new(self.db, 0, self.dimension)?;
        writer.del_item(&mut wtxn, id)?;
        let mut rng = StdRng::seed_from_u64(0);
        writer.build(&mut wtxn, &mut rng, None)?;
        wtxn.commit()?;
        Ok(())
    }

    pub fn find(&self, vector: &[f32]) -> Result<Vec<(u32, f32)>> {
        let wtxn = self.env.write_txn()?;
        let rtxn = self.env.read_txn()?;
        let reader = Reader::open(&wtxn, 0, self.db)?;
        let vectors = reader.nns_by_vector(&rtxn, vector, 1, None, None)?;
        Ok(vectors)
    }
}


#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    const TEST_DB_PATH: &str = ".db_test";

    #[test]
    fn test_insert_and_search() -> Result<()> {
        // Create a new Engine
        let engine_result = VectorStore::try_create(TEST_DB_PATH, 3);
        let engine = engine_result?;

        // Test insert
        let id = 1;
        let vector = vec![1.0, 2.0, 3.0];
        engine.insert(id, &vector)?;

        // Test search
        let search_result = engine.find(&vector)?;

        // Ensure the search result is not empty
        assert!(!search_result.is_empty());

        // Check if the inserted ID is present in the search result
        let found_id = search_result[0].0;
        assert_eq!(found_id, id);
        Ok(())
    }
}