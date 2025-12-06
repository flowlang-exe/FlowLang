use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::types::Value;

/// Inline cache entry for property/method lookups
#[derive(Clone, Debug)]
pub enum CacheEntry {
    /// Monomorphic cache - single type cached
    Monomorphic {
        type_name: String,
        cached_value: CachedLookup,
    },
    /// Polymorphic cache - multiple types cached (up to 4)
    Polymorphic {
        entries: Vec<(String, CachedLookup)>,
    },
    /// Megamorphic - too many types, fall back to slow path
    Megamorphic,
}

#[derive(Clone, Debug)]
pub enum CachedLookup {
    /// Property access result
    PropertyIndex(usize),
    /// Method pointer (for native functions)
    MethodPointer(String),
}

/// Global inline cache manager
pub struct InlineCache {
    /// Cache storage: (file, line, column) -> CacheEntry
    caches: Arc<RwLock<HashMap<(String, usize, usize), CacheEntry>>>,
    /// Statistics
    hits: Arc<RwLock<u64>>,
    misses: Arc<RwLock<u64>>,
}

impl InlineCache {
    pub fn new() -> Self {
        InlineCache {
            caches: Arc::new(RwLock::new(HashMap::new())),
            hits: Arc::new(RwLock::new(0)),
            misses: Arc::new(RwLock::new(0)),
        }
    }

    /// Try to lookup a property using the inline cache
    pub fn lookup_property(
        &self,
        location: (String, usize, usize),
        object: &Value,
        property: &str,
    ) -> Option<Value> {
        let type_name = object.type_name().to_string();
        
        // Try to read from cache
        if let Ok(caches) = self.caches.read() {
            if let Some(entry) = caches.get(&location) {
                match entry {
                    CacheEntry::Monomorphic { type_name: cached_type, cached_value } => {
                        if &type_name == cached_type {
                            // Cache hit!
                            if let Ok(mut hits) = self.hits.write() {
                                *hits += 1;
                            }
                            return self.extract_property(object, property, cached_value);
                        }
                    }
                    CacheEntry::Polymorphic { entries } => {
                        for (cached_type, cached_value) in entries {
                            if &type_name == cached_type {
                                if let Ok(mut hits) = self.hits.write() {
                                    *hits += 1;
                                }
                                return self.extract_property(object, property, cached_value);
                            }
                        }
                    }
                    CacheEntry::Megamorphic => {
                        // Too polymorphic, skip caching
                        return None;
                    }
                }
            }
        }

        // Cache miss
        if let Ok(mut misses) = self.misses.write() {
            *misses += 1;
        }

        None
    }

    /// Update the inline cache with a new lookup result
    pub fn update_cache(
        &self,
        location: (String, usize, usize),
        type_name: String,
        cached_value: CachedLookup,
    ) {
        if let Ok(mut caches) = self.caches.write() {
            let entry = caches.entry(location).or_insert_with(|| {
                CacheEntry::Monomorphic {
                    type_name: type_name.clone(),
                    cached_value: cached_value.clone(),
                }
            });

            match entry {
                CacheEntry::Monomorphic { type_name: existing_type, .. } => {
                    if existing_type != &type_name {
                        // Upgrade to polymorphic
                        let old_entry = entry.clone();
                        if let CacheEntry::Monomorphic { type_name: old_type, cached_value: old_value } = old_entry {
                            *entry = CacheEntry::Polymorphic {
                                entries: vec![
                                    (old_type, old_value),
                                    (type_name, cached_value),
                                ],
                            };
                        }
                    }
                }
                CacheEntry::Polymorphic { entries } => {
                    // Check if type already exists
                    if !entries.iter().any(|(t, _)| t == &type_name) {
                        if entries.len() < 4 {
                            // Add new type
                            entries.push((type_name, cached_value));
                        } else {
                            // Too many types, upgrade to megamorphic
                            *entry = CacheEntry::Megamorphic;
                        }
                    }
                }
                CacheEntry::Megamorphic => {
                    // Already megamorphic, do nothing
                }
            }
        }
    }

    /// Extract property value from cached lookup
    fn extract_property(&self, object: &Value, property: &str, cached: &CachedLookup) -> Option<Value> {
        match cached {
            CachedLookup::PropertyIndex(index) => {
                // For arrays, relics, etc.
                match object {
                    Value::Relic(map) => {
                        map.get(property).cloned()
                    }
                    _ => None,
                }
            }
            CachedLookup::MethodPointer(method_name) => {
                // Methods are handled differently - this is just a hint
                None
            }
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> (u64, u64, f64) {
        let hits = self.hits.read().map(|h| *h).unwrap_or(0);
        let misses = self.misses.read().map(|m| *m).unwrap_or(0);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }

    /// Clear all caches (useful for testing)
    pub fn clear(&self) {
        if let Ok(mut caches) = self.caches.write() {
            caches.clear();
        }
        if let Ok(mut hits) = self.hits.write() {
            *hits = 0;
        }
        if let Ok(mut misses) = self.misses.write() {
            *misses = 0;
        }
    }
}

impl Default for InlineCache {
    fn default() -> Self {
        Self::new()
    }
}
