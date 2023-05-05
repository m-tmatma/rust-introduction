use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let path = "sample.txt";
    let mut file = File::create(path)?;
    file.write(b"Hello rust world.\n")?;
    Ok(())
}
