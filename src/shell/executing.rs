use std::process;

use super::builtin;
use super::parsing::Command;
use super::{IOContext, Shell};

pub fn execute(sh: &mut Shell, io: &IOContext, ast: &Command) -> i32 {
    match ast {
        Command::Empty => 0,
        Command::Simple { keyword, args } => execute_simple(sh, io, keyword, args),
        Command::And { left, right } => execute_and(sh, io, left, right),
        Command::Or { left, right } => execute_or(sh, io, left, right),
        Command::Pipeline { left, right } => execute_pipeline(sh, io, left, right),
        // _ => {
        //     sh.eprintln("crsh: unimplemented functionality");
        //     -1
        // }
    }
}

fn execute_simple(sh: &mut Shell, io: &IOContext, keyword: &str, args: &[&str]) -> i32 {
    if let Some(builder) = builtin::get_builder(keyword) {
        match builder(args) {
            Ok(builtin) => builtin.run(sh),
            Err(e) => {
                sh.eprintln(e);
                -1
            }
        }
    } else if sh.find_on_path(keyword).is_some() {
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut cmd = process::Command::new(keyword);

        match cmd
            .stdin(io.input.clone())
            .stdout(io.output.clone())
            .stderr(io.error.clone())
            .args(&args)
            .spawn()
        {
            Ok(mut c) => match c.wait() {
                Ok(status) => status.code().unwrap_or(-1),
                Err(e) => {
                    sh.eprintln(format!("crsh: {e}"));
                    -1
                }
            },
            Err(e) => {
                sh.eprintln(format!("crsh: {e}"));
                -1
            }
        }
    } else {
        sh.eprintln(format!("crsh: command not found: {keyword}"));
        -1
    }
}

fn execute_and(sh: &mut Shell, io: &IOContext, left: &Command, right: &Command) -> i32 {
    let left_result = execute(sh, io, left);

    if left_result == 0 {
        execute(sh, io, right)
    } else {
        left_result
    }
}

fn execute_or(sh: &mut Shell, io: &IOContext, left: &Command, right: &Command) -> i32 {
    let left_result = execute(sh, io, left);

    if left_result != 0 {
        execute(sh, io, right)
    } else {
        left_result
    }
}

fn execute_pipeline(sh: &mut Shell, io: &IOContext, left: &Command, right: &Command) -> i32 {
    let left_io = IOContext {
        input: io.input.clone(),
        output: io.output.clone(),
        error: io.error.clone(),
    };

    let right_io = IOContext {
        input: io.input.clone(),
        output: io.output.clone(),
        error: io.error.clone(),
    };

    let _ = execute(sh, &left_io, left);
    execute(sh, &right_io, right)
}
