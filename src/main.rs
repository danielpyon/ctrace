use std::path::Path;
use std::fs::File;
use std::io::Read;

// sudo strace -tt -p <pid>
// sudo dmesg -w | grep audit
// ./compile.sh
// sudo apparmor -r /etc/apparmor.d/home.debian.Downloads.test

// adjustable interval
// colorful output

#[derive(Default)]
struct AuditLogEntry {
    cap: String,
    time: f64
}

fn parse_dmesg(s: &str) {
    // store ordered list of structs, each containing which capability was triggered (eg: net_raw)
    let mut audit_log_entries: Vec<AuditLogEntry> = Vec::new();

    for line in s.lines() {
        // TODO: parse the data and add it to v
        let time_str = line.find("audit(")
            .and_then(|start| {
                let after_audit = &line[start+6..];
                after_audit.find(':')
                    .map(|end| &after_audit[..end])
            });
        
        let mut entry = AuditLogEntry::default();

        entry.time = match time_str {
            Some(t) => {
                match t.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("failed to parse {} to double", s);
                        0.0
                    }
                }
            },
            None => {
                println!("failed to find unix timestamp in audit log");
                0.0
            },
        };

        entry.cap = line.find("capname=\"")
            .and_then(|start| {
                line[start+9..].find("\"")
                    .map(|end| &line[start+9..][..end])
            }).unwrap_or("").to_string();

        println!("found audit log entry (cap={}, time={})", entry.cap, entry.time);
        audit_log_entries.push(entry);
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
