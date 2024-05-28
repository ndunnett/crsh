use libc::{EXIT_FAILURE, WUNTRACED};
use std::ffi::CString;

fn _execute(args: &[&str], status: &mut i32) {
    match unsafe { libc::fork() } {
        -1 => {
            *status = -1;
        }
        0 => {
            let c_strings = args
                .iter()
                .map(|&arg| CString::new(arg).unwrap())
                .collect::<Vec<_>>();

            let argv = c_strings
                .iter()
                .map(|arg| arg.as_ptr())
                .chain([std::ptr::null()])
                .collect::<Vec<_>>();

            unsafe {
                if libc::execvp(argv[0], argv.as_ptr()) == -1 {
                    *status = -1;
                }

                libc::exit(EXIT_FAILURE);
            };
        }
        pid => unsafe {
            libc::waitpid(pid, status, WUNTRACED);
        },
    }
}

pub fn execute(args: &[&str]) -> Result<(), String> {
    let mut status = 0;
    _execute(args, &mut status);

    match status {
        0 => Ok(()),
        256 => Err(format!("failed to find \"{}\"", args[0])),
        _ => Err(format!("exit code {}", status)),
    }
}
