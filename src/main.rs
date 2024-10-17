use std::path::Path;
use std::fs::File;
use std::io::Read;

// sudo strace -tt -p <pid>
// sudo dmesg -w | grep audit
// ./compile.sh
// sudo apparmor -r /etc/apparmor.d/home.debian.Downloads.test

// adjustable interval
// colorful output

struct AuditLogEntry {
    cap: String,
    time: f64
}

fn parse_dmesg(s: &str) {
    // store ordered list of structs, each containing which capability was triggered (eg: net_raw)
    let v: Vec<AuditLogEntry> = Vec::new();

    for line in s.lines() {
        println!("{}", line);
        // TODO: parse the data and add it to v
    }
}

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
        Ok(_) => parse_dmesg(&s),
    }
}
