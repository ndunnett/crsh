use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub(super) profile_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut profile_path = std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let mut p = std::env::var("HOME")
                    .ok()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| "/".into());

                p.push(".config");
                p
            });

        profile_path.push("crsh");

        if !profile_path.is_dir() {
            _ = std::fs::create_dir_all(&profile_path);
        }

        Self { profile_path }
    }
}
