use anyhow::Error;

pub fn run(verbose: bool) -> Result<(), Error> {
    println!("currently available probes");
    if verbose {
        println!("some verbose output");
    }

    Ok(())
}
