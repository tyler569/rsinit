use std::env;
use std::io;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    cat(&args).unwrap();
}

fn cat(files: &[String]) -> io::Result<()> {
    for file in files {
        let mut f = fs::File::open(file)?;
        io::copy(&mut f, &mut io::stdout())?;
    }
    Ok(())
}

