use directories::ProjectDirs;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct ProjectDirSINGLETON {
    pub dir: ProjectDirs,
}

// Global singleton, thread-safe
pub static PROJECT_INSTANCE: Lazy<Mutex<ProjectDirSINGLETON>> = Lazy::new(|| {
    Mutex::new(ProjectDirSINGLETON {
        dir: ProjectDirs::from("com", "Jayden", "fetch").unwrap(),
    })
});

macro_rules! proj_dir {
    () => {
        PROJECT_INSTANCE.lock().unwrap().dir
    };
}
