use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, RwLock};
use std::thread;
use base64::{engine::general_purpose, Engine as _};
use crate::constants::db::*;

pub struct Kline {
    store: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
    log: File,
}

impl Kline {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;

        let mut store = HashMap::new();
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(3, ' ').collect();

             match parts.as_slice() {
                ["put", key_b64, value_b64] => {
                    if let (Ok(key), Ok(value)) = (
                        general_purpose::STANDARD.decode(key_b64),
                        general_purpose::STANDARD.decode(value_b64),
                    ) {
                        store.insert(key, value);
                    }
                }
                ["delete", key_b64] => {
                    if let Ok(key) = general_purpose::STANDARD.decode(key_b64) {
                        store.remove(&key);
                    }
                }
                _ => {}
            }
        }

        let store_arc = Arc::new(RwLock::new(store));
        let _log = OpenOptions::new().append(true).create(true).open(path)?;

        // compaction thread

        let path_str = path.to_string();
        let store_for_thread = Arc::clone(&store_arc);

        thread::spawn(move || loop {
            thread::sleep(COMPACTION_INTERVAL);
            let temp_path = format!("{}{}", path_str, TEMP_FILE_SUFFIX);
            if let Ok(store) = store_for_thread.read() {
                if let Ok(mut temp_file) = File::create(&temp_path) {
                    for (key, val) in store.iter() {
                        let key_b64 = general_purpose::STANDARD.encode(key);
                        let val_b64 = general_purpose::STANDARD.encode(val);
                        let _ = writeln!(temp_file, "put {} {}", key_b64, val_b64);
                    }
                    // Atomically replace old log
                    let _ = std::fs::rename(&temp_path, &path_str);
                }
            }
        });


        // reopen file for append (because BufReader uses read-only)
        let log = OpenOptions::new()
            .append(true)
            .open(path)?;

        Ok(Kline { store: store_arc, log })
    }


    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> std::io::Result<()> {
        let key_b64 = general_purpose::STANDARD.encode(&key);
        let value_b64 = general_purpose::STANDARD.encode(&value);
        writeln!(self.log, "put {} {}", key_b64, value_b64)?;
        self.log.flush()?;

        let mut store = self.store.write().unwrap();
        store.insert(key, value);
        Ok(())
    }



    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let store = self.store.read().unwrap();
        store.get(key).cloned()
    }


     pub fn delete(&mut self, key: &[u8]) -> std::io::Result<()> {
        let key_b64 = general_purpose::STANDARD.encode(key);
        writeln!(self.log, "delete {}", key_b64)?;
        self.log.flush()?;
        let mut store = self.store.write().unwrap();
        store.remove(key);
        Ok(())
    }

    pub fn compact(&mut self) -> std::io::Result<()> {
        let temp_path = format!("{}{}", DEFAULT_DB_FILE, TEMP_FILE_SUFFIX);
        let mut temp_file = File::create(&temp_path)?;

        let store = self.store.read().unwrap();
        for (key, value) in store.iter() {
            let key_b64 = general_purpose::STANDARD.encode(key);
            let value_b64 = general_purpose::STANDARD.encode(value);
            writeln!(temp_file, "put {} {}", key_b64, value_b64)?;
        }

        std::fs::rename(&temp_path, DEFAULT_DB_FILE)?;
        self.log = OpenOptions::new().append(true).open(DEFAULT_DB_FILE)?;
        Ok(())
    }

    pub fn keys(&self) -> Vec<Vec<u8>> {
        let store = self.store.read().unwrap();
        store.keys().cloned().collect()
    }
    
    pub fn clear(&mut self) -> std::io::Result<()> {
        {
            let mut store = self.store.write().unwrap();
            store.clear();
        } // store lock is dropped here
        self.compact() // write empty state to disk
    }
}
