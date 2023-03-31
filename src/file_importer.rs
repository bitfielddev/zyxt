use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use color_eyre::{eyre::eyre, Result};
use once_cell::sync::Lazy;
use smol_str::SmolStr;

static FILE_CACHE: Lazy<Mutex<HashMap<SmolStr, Arc<str>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn import_file(file: &Path) -> Result<Arc<str>> {
    Ok(Arc::clone(
        FILE_CACHE
            .lock()
            .map_err(|a| eyre!("{a:?}"))?
            .entry(file.to_string_lossy().into())
            .or_insert_with(|| Arc::from(std::fs::read_to_string(file).unwrap())),
    ))
}

pub fn register_input(name: &SmolStr, input: &str) -> Result<Arc<str>> {
    let mut cache = FILE_CACHE.lock().map_err(|a| eyre!("{a:?}"))?;
    let input = Arc::from(input);
    cache.insert(name.to_owned(), Arc::clone(&input));
    Ok(input)
}

pub fn get_input(name: &SmolStr) -> Result<Option<Arc<str>>> {
    let cache = FILE_CACHE.lock().map_err(|a| eyre!("{a:?}"))?;
    let res = cache.get(name).cloned();
    drop(cache);
    if let Some(res) = res {
        Ok(Some(res))
    } else {
        PathBuf::try_from(name.to_string())
            .ok()
            .map(|p| import_file(&p))
            .transpose()
    }
}
