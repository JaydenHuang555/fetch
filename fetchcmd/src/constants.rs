use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use once_cell::sync::Lazy;

use crate::proj_dir::PROJECT_INSTANCE;

pub struct Constants {
    pub profiles_path: PathBuf,
    pub cache_path: PathBuf,
}

lazy_static::lazy_static! {

    pub static ref INSTANCE: Lazy<Mutex<Constants>> = Lazy::new(|| {
        let proj = PROJECT_INSTANCE.lock().unwrap().dir.clone();
        let profiles_path = proj.config_dir().join("profiles");
        let cache_path = proj.cache_dir().to_path_buf();
        Mutex::new(
            Constants {
                profiles_path: profiles_path,
                cache_path: cache_path
            }
        )
    });
}

macro_rules! constants_instance {
    () => {
        INSTANCE.lock().unwrap()
    };
}

pub(crate) use constants_instance;
