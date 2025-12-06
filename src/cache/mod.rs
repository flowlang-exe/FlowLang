use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use sha2::{Sha256, Digest};
use crate::parser::ast::Program;
use crate::error::FlowError;

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from(".flowlang/ast");
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }
        CacheManager { cache_dir }
    }

    pub fn load(&self, file_path: &Path, source: &str) -> Option<Program> {
        let cache_path = self.get_cache_path(file_path);
        
        if !cache_path.exists() {
            return None;
        }

        // Read cache file
        // Format: [Hash (32 bytes)][Bincode Encoded AST]
        let mut file = match fs::File::open(&cache_path) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return None;
        }

        if buffer.len() < 32 {
            return None; // Invalid cache
        }

        // Verify hash
        let stored_hash = &buffer[0..32];
        let current_hash = self.compute_hash(source);
        
        if stored_hash != current_hash.as_slice() {
            return None; // Source changed, cache invalid
        }

        // Deserialize AST
        let ast_data = &buffer[32..];
        match bincode::deserialize(ast_data) {
            Ok(program) => Some(program),
            Err(_) => None,
        }
    }

    pub fn save(&self, file_path: &Path, source: &str, program: &Program) -> Result<(), FlowError> {
        let cache_path = self.get_cache_path(file_path);
        
        // Ensure directory exists
        if let Some(parent) = cache_path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }

        let hash = self.compute_hash(source);
        let ast_data = bincode::serialize(program).map_err(|e| {
            FlowError::runtime(&format!("Failed to serialize AST: {}", e), 0, 0)
        })?;

        let mut file = fs::File::create(&cache_path).map_err(|e| {
            FlowError::runtime(&format!("Failed to create cache file: {}", e), 0, 0)
        })?;

        // Write hash then data
        file.write_all(&hash).map_err(|e| {
            FlowError::runtime(&format!("Failed to write cache: {}", e), 0, 0)
        })?;
        
        file.write_all(&ast_data).map_err(|e| {
            FlowError::runtime(&format!("Failed to write cache: {}", e), 0, 0)
        })?;

        Ok(())
    }

    fn compute_hash(&self, source: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        hasher.finalize().to_vec()
    }

    fn get_cache_path(&self, file_path: &Path) -> PathBuf {
        // Create a unique filename based on the absolute path hash to avoid collisions
        // and handle files with same name in different dirs
        let abs_path = fs::canonicalize(file_path).unwrap_or(file_path.to_path_buf());
        let path_str = abs_path.to_string_lossy();
        
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        let path_hash = hex::encode(hasher.finalize());
        
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
            
        self.cache_dir.join(format!("{}_{}.flowast", filename, &path_hash[0..8]))
    }
}
