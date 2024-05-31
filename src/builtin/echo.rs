pub fn echo(args: &[&str]) -> Result<(), ()> {
    println!("{}", args.join(" "));
    Ok(())
}
