use std::env;
use std::io;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        0 | 1 => list_dir("."),
        _ => list_dir(&args[1]),
    }.unwrap();
}

fn list_dir(name: &str) -> io::Result<()> {
    let entries = fs::read_dir(name)?;
    for entry in entries {
        println!("{}", entry?.path().to_str().unwrap());
    }
    Ok(())
}
