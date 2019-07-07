use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_memory_usage_header() {
    println!("VmSize(kB), VmRSS(kB)");
}

fn extract_size(s: &str) -> u64 {
    let start = s.find(':').unwrap() + 1;
    let end = s.len() - 3;
    s[start..end].trim_start().parse().unwrap()
}

pub fn print_memory_usage() {
    let f = File::open("/proc/self/status").unwrap();
    let file = BufReader::new(&f);
    let mut size = 0;
    let mut rss = 0;
    for line in file.lines() {
        let s = line.unwrap();
        if s.starts_with("VmSize:") {
            size = extract_size(&s);
        } else if s.starts_with("VmRSS:") {
            rss = extract_size(&s);
        }
    }
    println!("{}, {}", size, rss);
}
