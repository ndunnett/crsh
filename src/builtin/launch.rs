use std::process::Command;

pub fn launch(keyword: &str, args: &[&str]) -> Result<(), ()> {
    let mut command = Command::new(keyword);

    match command.args(args).spawn() {
        Ok(mut c) => match c.wait() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(())
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(())
            }
        },
        Err(e) => {
            eprintln!("error: {}", e);
            Err(())
        }
    }
}
