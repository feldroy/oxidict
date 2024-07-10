use pyo3::prelude::*;
use std::collections::HashMap;

/// Python OxiDict class implemented as Rust struct.
#[pyclass]
struct OxiDict {
    hashmap: HashMap<String, String>,
}

#[pymethods]
impl OxiDict {
    #[new]
    fn new() -> Self {
        OxiDict {
            hashmap: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }

    fn get(&self, key: String) -> Option<&String> {
        self.hashmap.get(&key)
    }

    fn remove(&mut self, key: String) -> Option<String> {
        self.hashmap.remove(&key)
    }

    fn len(&self) -> usize {
        self.hashmap.len()
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.hashmap)
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn oxidict(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OxiDict>()?;
    Ok(())
}
