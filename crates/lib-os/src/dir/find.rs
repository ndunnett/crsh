use std::{
    path::{Path, PathBuf},
    slice::Iter,
    sync::OnceLock,
};

#[cfg(not(target_os = "windows"))]
pub fn find_on_path<P: AsRef<Path>>(keyword: P) -> Option<PathBuf> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(&keyword))
        .find(|path| path.is_file())
}

#[cfg(target_os = "windows")]
pub fn find_on_path<P: AsRef<Path>>(keyword: P) -> Option<PathBuf> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|dir| dir.join(&keyword))
        .flat_map(|path| path_ext().map(move |ext| path.clone().with_extension(ext)))
        .find(|path| path.is_file())
}

#[cfg(target_os = "windows")]
fn path_ext() -> Iter<'static, String> {
    static PATHEXT: OnceLock<Vec<String>> = OnceLock::new();

    PATHEXT
        .get_or_init(|| {
            std::env::var("PATHEXT")
                .unwrap_or(".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.CPL".into())
                .split(';')
                .map(|ext| ext.trim_start_matches('.').to_lowercase().to_string())
                .collect::<Vec<_>>()
        })
        .iter()
}
