use anyhow::Error;

pub const COMMAND_NAME: &str = "install";

pub fn run() -> Result<(), Error> {
    println!("{}", COMMAND_NAME);
    Ok(())
}
