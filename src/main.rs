use std::env;
use std::io;
use std::thread;
use io_streams;
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use anyhow::Result;

// based on example from https://docs.rs/portable-pty/0.4.0/portable_pty/index.html
// copyright Thomas van der Berg, 2021

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();
    let cmdname = if args.len() >= 2 { &args[1] } else { "sh" };

    // Use the native pty implementation for the system
    let pty_system = native_pty_system();

    // Create a new pty
    let mut pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new(cmdname);
    let mut child = pair.slave.spawn_command(cmd)?;

    // Read and parse output from the pty with reader
    let mut reader = pair.master.try_clone_reader()?;

    // copy bash output to stdout
    thread::spawn(move || {
        io::copy(&mut reader, &mut io_streams::StreamWriter::stdout().unwrap()).unwrap();
    });

    // copy stdin to pty stdin
    thread::spawn(move || {
        io::copy(&mut io_streams::StreamReader::stdin().unwrap(), &mut pair.master).unwrap();
    });

    // quit when shell exits
    child.wait()?;

    Ok(())
}
