use std::env;

use crate::system::dirs;

pub fn cd(args: &[&str]) -> Result<(), ()> {
    let path = args.iter().next().map_or_else(dirs::home, |&s| {
        if s.starts_with('~') {
            s.replacen('~', &dirs::home(), 1)
        } else {
            s.into()
        }
    });

    if let Err(e) = env::set_current_dir(path) {
        eprintln!("cd: {}", e);
        Err(())
    } else {
        Ok(())
    }
}
