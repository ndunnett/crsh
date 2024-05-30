pub fn echo(args: &[&str]) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}
