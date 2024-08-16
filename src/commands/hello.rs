use super::*;

#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: Args) -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
