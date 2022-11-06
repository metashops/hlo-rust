use std::fs::File;
use std::io::Read;
use std::io;
fn main() {
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("error: {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("src/text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}