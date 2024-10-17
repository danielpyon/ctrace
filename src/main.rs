use std::path::Path;
use std::fs::File;
use std::io::Read;

// sudo strace -tt -p <pid>
// sudo dmesg -w | grep audit

// ./compile.sh

// sudo apparmor -r /etc/apparmor.d/home.debian.Downloads.test

/*
fn parse_dmesg() {

}

fn parse_strace() {

}
*/

fn main() {
    let dmesg_path = Path::new("dmesg.txt");
    let display = dmesg_path.display();

    let mut file = match File::open(&dmesg_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains\n{}", display, s),
    }

    // parse_strace(&mut s);
}
