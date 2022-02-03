use anyhow::Error;

pub const COMMAND_NAME: &str = "start";

pub fn run() -> Result<(), Error> {
    println!("{}", COMMAND_NAME);
    Ok(())
}
