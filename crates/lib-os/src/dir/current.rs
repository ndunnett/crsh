pub fn current() -> String {
    format!(
        "{}",
        std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
    )
}
