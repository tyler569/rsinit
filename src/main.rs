use lazy_static::lazy_static;
use regex::Regex;
use std::{
    ffi::CString,
    fs::{read_dir, File},
    io::{self, prelude::*},
    process::Command,
    thread::sleep,
    time::Duration,
};

//  #define MS_REMOUNT   32      /* Alter flags of a mounted FS */
const MS_REMOUNT: usize = 32;
extern "C" {
    // int mount(
    //     const char *source,
    //     const char *target,
    //     const char *filesystemtype,
    //     unsigned long mountflags,
    //     const void *data
    // );
    fn mount(
        source: *const libc::c_char,
        target: *const libc::c_char,
        typ: *const libc::c_char,
        flags: usize,
        data: *const libc::c_char,
    ) -> libc::c_int;
}

fn rs_mount(
    source: &str,
    target: &str,
    typ: &str,
    flags: usize,
) -> Result<(), nix::Error> {
    let source = CString::new(source).unwrap();
    let target = CString::new(target).unwrap();
    let typ = CString::new(typ).unwrap();
    let result = unsafe {
        mount(
            source.as_ptr(),
            target.as_ptr(),
            typ.as_ptr(),
            flags,
            std::ptr::null(),
        )
    };

    if result == 0 { Ok(()) } else { Err(nix::Error::last()) }
}

fn tokenize(st: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#""(?:\\"[^"])*"|'[^']*'|[^()\[\]{}|<>;& \n]+|[^ \n]"#)
                .unwrap();
    }
    RE.find_iter(st).map(|m| m.as_str()).collect()
}

const DEBUG_SHELL: bool = false;

fn list_dir(name: &str) {
    if let Ok(entries) = read_dir(name) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.path().to_str().unwrap());
            }
        }
    } else {
        println!("error reading {}\n", name);
    }
}

fn cat(files: &[&str]) {
    for file in files {
        let mut f = match File::open(file) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("error opening {}: {:?}", file, err);
                continue;
            }
        };
        if let Err(err) = io::copy(&mut f, &mut io::stdout()) {
            eprintln!("error reading {}: {:?}", file, err);
        }
    }
}

fn exec(program: &str, args: &[&str]) {
    let mut c = Command::new(program);
    c.args(args);
    let mut child = match c.spawn() {
        Ok(child) => child,
        Err(err) => {
            eprintln!("failed to run {}: {:?}", program, err);
            return;
        }
    };
    let code = match child.wait() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("failed to wait on {}: {:?}", program, err);
            return;
        }
    };
    if !code.success() {
        eprintln!("{} ran, but indicated failure: {:?}", program, code);
    }
}

fn handle_line(line: String) {
    let tokens = tokenize(&line);
    if DEBUG_SHELL {
        println!("{:?}", tokens);
    }

    if let Some((cmd, args)) = tokens.split_first() {
        match *cmd {
            "echo" => println!("{}", args.join(" ")),
            "ls" => list_dir(args.first().cloned().unwrap_or(".")),
            "cat" => cat(args),
            _ => exec(cmd, args),
        }
    }
}

fn do_shell() {
    fn prompt() {
        eprint!("$ ");
    }

    prompt();
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            handle_line(line)
        } else {
            println!("{:?}", line);
        }
        prompt();
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        println!("{}", info);
        loop {
            sleep(Duration::new(1, 0));
        }
    }));

    for _ in 0..10 {
        println!("Hello, world!");
    }

    println!("remount root: {:?}", rs_mount("/dev/vda", "/", "", MS_REMOUNT));
    println!("mount proc: {:?}", rs_mount("", "/proc", "proc", 0));

    std::env::set_current_dir("/root");

    do_shell();

    panic!("init tried to return");
}