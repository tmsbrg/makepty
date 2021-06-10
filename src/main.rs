use std::io;
use std::thread;
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use anyhow::Result;


fn main() -> Result<()> {
    // Use the native pty implementation for the system
    let pty_system = native_pty_system();

    // Create a new pty
    let mut pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        // Not all systems support pixel_width, pixel_height,
        // but it is good practice to set it to something
        // that matches the size of the selected font.  That
        // is more complex than can be shown here in this
        // brief example though!
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("bash");
    let _child = pair.slave.spawn_command(cmd)?;

    // Read and parse output from the pty with reader
    let mut reader = pair.master.try_clone_reader()?;

    // copy bash output to stdout
    thread::spawn(move || {
        io::copy(&mut reader, &mut io::stdout()).unwrap();
    });

    // copy stdin to pty stdin
    io::copy(&mut io::stdin(), &mut pair.master)?;

    Ok(())
}
