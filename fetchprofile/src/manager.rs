use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{error::ProfileManagerError, profile::Profile};

pub struct ProfileManager {
    pub profile_map: BTreeMap<String, Profile>,
    pub dir_buff: PathBuf,
}

impl ProfileManager {
    pub fn is_valid(dir: &Path, info: &Path) -> bool {
        let info_path_buff = dir.join(info);
        let info_path = info_path_buff.as_path();

        return info_path.is_file() && dir.is_dir();
    }

    pub fn load(dir: &Path) -> Result<Self, ProfileManagerError> {
        let mut this = Self {
            profile_map: BTreeMap::new(),
            dir_buff: dir.to_path_buf(),
        };

        let info_path_buff = dir.join("info");
        let info_path = info_path_buff.as_path();

        if !Self::is_valid(dir, info_path) {
            if let Err(e) = Self::generate(dir, info_path) {
                return Err(e);
            }
        }

        match this.update() {
            Ok(()) => Ok(this),
            Err(e) => Err(e),
        }
    }

    fn generate(dir: &Path, info: &Path) -> Result<(), ProfileManagerError> {
        if !dir.exists() {
            if let Err(e) = fs::create_dir_all(dir) {
                println!("Unable to create dir {}", dir.display());
                return Err(ProfileManagerError::DirectoryIO(e));
            }
        } else if dir.is_file() {
            return Err(ProfileManagerError::InvalidDirectory(
                "Given Directory to not be a file",
            ));
        }

        if let Err(e) = fs::write(info, "") {
            return Err(ProfileManagerError::FileIO(e));
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), ProfileManagerError> {
        let path = self.dir_buff.as_path();
        let entries;
        let read_dir_output = fs::read_dir(path);
        if let Ok(output) = read_dir_output {
            entries = output;
        } else {
            return Err(ProfileManagerError::DirectoryIO(
                read_dir_output.unwrap_err(),
            ));
        }
        for output in entries {
            if output.is_err() {
                return Err(ProfileManagerError::FileIO(output.unwrap_err()));
            }
            let entry = output.unwrap();
            match entry.path().extension() {
                Some(extension) => {
                    if extension.to_os_string().as_encoded_bytes() != b"json" {
                        continue;
                    }
                }
                None => {
                    continue;
                }
            }
            if let Ok(name) = entry.file_name().into_string() {
                if self.profile_map.contains_key(&name) {
                    continue;
                }
                let contents;
                match fs::read_to_string(entry.path()) {
                    Ok(read) => {
                        contents = read;
                    }
                    Err(e) => return Err(ProfileManagerError::FileIO(e)),
                }
                match serde_json::from_str(contents.as_str()) {
                    Ok(p) => {
                        let profile: Profile = p;
                        self.profile_map.insert(profile.key.clone(), profile);
                    }
                    Err(e) => return Err(ProfileManagerError::DeserializeError(e)),
                }
            }
        }
        Result::Ok(())
    }

    pub fn add_profile(
        &self,
        profile: &Profile,
        force_flush: bool,
    ) -> Result<bool, ProfileManagerError> {
        let mut path_buff = self.dir_buff.join(profile.key.clone());
        path_buff.set_extension("json");
        let path = path_buff.as_path();

        // TODO: check if the path is a valid profile file
        if !force_flush && path.exists() && path.is_file() {
            return Ok(false);
        }

        if path.exists() && !path.is_file() {
            return Err(ProfileManagerError::CollisionDetected(
                crate::error::CollisionType::FileName(path.to_str().unwrap().to_string()),
            ));
        }

        match serde_json::to_string(profile) {
            Ok(contents) => {
                if let Err(e) = fs::write(path, contents) {
                    return Err(ProfileManagerError::FileIO(e));
                }
                return Ok(true);
            }
            Err(e) => {
                return Err(ProfileManagerError::SerializeError(e));
            }
        }
    }

    pub fn get_profile(&self, key: String) -> Option<Profile> {
        match self.profile_map.get(&key) {
            Some(profile) => Some(profile.clone()),
            None => None,
        }
    }
}
