use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;
use std::{collections::HashMap, sync::Arc};

use sway_error::error::CompileError;
use sway_error::warning::CompileWarning;

use crate::Programs;

pub type ModulePath = Arc<PathBuf>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModuleCacheKey {
    pub path: Arc<PathBuf>,
    pub include_tests: bool,
}

impl ModuleCacheKey {
    pub fn new(path: Arc<PathBuf>, include_tests: bool) -> Self {
        Self {
            path,
            include_tests,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleCacheEntry {
    pub path: ModulePath,
    pub modified_time: Option<SystemTime>,
    pub hash: u64,
    pub dependencies: Vec<ModulePath>,
    pub include_tests: bool,
}

pub type ModuleCacheMap = HashMap<ModuleCacheKey, ModuleCacheEntry>;

#[derive(Clone, Debug)]
pub struct ProgramsCacheEntry {
    pub path: ModulePath,
    pub programs: Programs,
    pub handler_data: (Vec<CompileError>, Vec<CompileWarning>),
}

pub type ProgramsCacheMap = HashMap<ModulePath, ProgramsCacheEntry>;

#[derive(Debug, Default)]
pub struct QueryEngine {
    parse_module_cache: RwLock<ModuleCacheMap>,
    programs_cache: RwLock<ProgramsCacheMap>,
}

impl Clone for QueryEngine {
    fn clone(&self) -> Self {
        Self {
            parse_module_cache: RwLock::new(self.parse_module_cache.read().unwrap().clone()),
            programs_cache: RwLock::new(self.programs_cache.read().unwrap().clone()),
        }
    }
}

impl QueryEngine {
    pub fn get_parse_module_cache_entry(&self, path: &ModuleCacheKey) -> Option<ModuleCacheEntry> {
        let cache = self.parse_module_cache.read().unwrap();
        cache.get(path).cloned()
    }

    pub fn insert_parse_module_cache_entry(&self, entry: ModuleCacheEntry) {
        let mut cache = self.parse_module_cache.write().unwrap();
        let path = entry.path.clone();
        let include_tests = entry.include_tests;

        let key = ModuleCacheKey::new(path, include_tests);
        cache.insert(key, entry);
    }

    pub fn get_programs_cache_entry(&self, path: &Arc<PathBuf>) -> Option<ProgramsCacheEntry> {
        let cache = self
            .programs_cache
            .read()
            .expect("Failed to read programs cache");
        cache.get(path).cloned()
    }

    pub fn insert_programs_cache_entry(&self, entry: ProgramsCacheEntry) {
        let mut cache = self.programs_cache.write().unwrap();
        cache.insert(entry.path.clone(), entry);
    }
}
