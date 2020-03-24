extern crate nix;
extern crate tempfile;

use nix::sys::stat;
use nix::unistd;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time;

fn main() {
    let fifo_path = Path::new("/tmp/foo.pipe");
    let mut file = OpenOptions::new().read(true).open(&fifo_path).unwrap();
    println!("Opened a file");
    let mut contents = String::new();
    loop {
        println!("Begin loop");
        file.read_to_string(&mut contents).unwrap();
        println!("Received: {}", contents);
        sleep(time::Duration::from_secs(3));
    }
}
