use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub fn get_etc_path() -> PathBuf {
    return std::env::current_dir().unwrap();
}
#[cfg(target_os = "linux")]
pub fn get_etc_path() -> PathBuf {
    if std::path::Path::new("/etc/tust_newspaper").exists() {
        return PathBuf::from("/etc/tust_newspaper");
    }
    let mut current_dir=std::env::current_dir().unwrap();
    current_dir.pop();
    current_dir.push("etc");
    current_dir.push("tust_newspaper");
    if current_dir.exists() {
        return current_dir;
    }
    return std::env::current_dir().unwrap();
}
#[cfg(target_os = "macos")]
pub fn get_etc_path() -> PathBuf {
    let config_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .expect("Failed to get config dir");
    return config_dir;
}
pub struct ConfigError{
    message: String,
}
impl ConfigError {
    pub fn new(message: &str) -> ConfigError {
        return ConfigError{
            message: message.to_string(),
        }
    }
}
impl std::error::Error for ConfigError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}