pub fn home<S: AsRef<str>>(user: S) -> String {
    format!(
        "{}",
        homedir::home(user)
            .unwrap_or_default()
            .unwrap_or_default()
            .to_string_lossy()
    )
}

pub fn my_home() -> String {
    format!(
        "{}",
        homedir::my_home()
            .unwrap_or_default()
            .unwrap_or_default()
            .to_string_lossy()
    )
}
