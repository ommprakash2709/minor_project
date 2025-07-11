use chrono::{DateTime, Utc};
use sled::Db;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Meta {
    pub mtime: i64,
    pub hash: String,
}

pub struct Index {
    tree: Db,
}
impl Index {
    pub fn open(root: &Path) -> sled::Result<Self> {
        let db = sled::open(root.join(".deduper/index"))?;
        Ok(Self { tree: db })
    }
    /// Returns true if file was unchanged since last run.
    pub fn is_fresh(&self, path: &Path, mtime: i64) -> bool {
        self.tree
            .get(path.as_os_str().as_encoded_bytes())
            .ok()
            .flatten()
            .and_then(|v| bincode::deserialize::<Meta>(&v).ok())
            .map(|m| m.mtime == mtime)
            .unwrap_or(false)
    }
    pub fn upsert(&self, path: &Path, mtime: i64, hash: &str) {
        let meta = Meta {
            mtime,
            hash: hash.to_string(),
        };
        let _ = self.tree.insert(
            path.as_os_str().as_encoded_bytes(),
            bincode::serialize(&meta).unwrap(),
        );
    }
}
