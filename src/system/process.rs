use std::ffi::CString;

pub enum ForkResult {
    Child,
    Parent(i32),
}

pub fn fork() -> Result<ForkResult, String> {
    let result = unsafe { libc::fork() };

    match result {
        -1 => Err("failed to fork process".to_string()),
        0 => Ok(ForkResult::Child),
        _ => Ok(ForkResult::Parent(result)),
    }
}

pub fn execvp(command: &str, args: &[&str]) -> Result<(), String> {
    let file = CString::new(command).unwrap();
    let c_args = args
        .iter()
        .map(|&arg| CString::new(arg).unwrap())
        .collect::<Vec<_>>();

    let mut argv = vec![file.as_ptr()];
    argv.extend(c_args.iter().map(|arg| arg.as_ptr()));
    argv.push(std::ptr::null());

    let result = unsafe { libc::execvp(file.as_ptr(), argv.as_ptr()) };

    // todo: proper error handling
    if result == -1 {
        Err("failed to execute".to_string())
    } else {
        Ok(())
    }
}

pub fn waitpid(pid: i32) -> Result<(), String> {
    // todo: proper error handling
    let mut status: libc::c_int = 0;

    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }

    Ok(())
}
