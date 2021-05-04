use lazy_static::lazy_static;
use regex::Regex;
use std::{
    ffi::CString,
    io::{self, prelude::*},
    process::Command,
    thread::sleep,
    time::Duration,
};

const MS_REMOUNT: usize = 32;
extern "C" {
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

fn exec(program: &str, args: &[&str]) -> io::Result<()> {
    let status = Command::new(program)
        .args(args)
        .spawn()?
        .wait()?;
    if !status.success() {
        eprintln!("{} ran, but indicated failure: {:?}", program, status);
    }
    Ok(())
}

fn handle_line(line: String) -> io::Result<()> {
    let tokens = tokenize(&line);
    if DEBUG_SHELL {
        println!("{:?}", tokens);
    }

    if let Some((cmd, args)) = tokens.split_first() {
        match *cmd {
            "echo" => println!("{}", args.join(" ")),
            _ => exec(cmd, args)?,
        }
    }
    Ok(())
}

fn do_shell() {
    fn prompt() {
        eprint!("$ ");
    }

    prompt();
    for line in io::stdin().lock().lines() {
        if let Err(err) = line.and_then(handle_line) {
            eprintln!("err: {:?}", err);
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

    std::env::set_current_dir("/root").unwrap();

    do_shell();

    panic!("init tried to return");
}
