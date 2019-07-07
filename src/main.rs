use getopts::Options;
use oracle::Connection;
use std::env;
use std::thread;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::*;

const USERNAME: &str = "username";
const PASSWORD: &str = "password";
const DATABASE: &str = "database";

fn connect_db(connect: bool) {
    if connect {
        let conn = Connection::connect(USERNAME, PASSWORD, DATABASE, &[]).unwrap();
        conn.query("select systimestamp from dual", &[]).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("t", "", "use thread");
    opts.optflag("n", "", "don't connect. just for test of printing memory usage");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let use_thread = matches.opt_present("t");
    let connect = !matches.opt_present("n");
    let num_connections = 30;
    let max_iteration_count = 1000;

    print_memory_usage_header();
    for _ in 0..max_iteration_count {
        if use_thread {
            let mut handles = Vec::new();
            for _ in 0..num_connections {
                handles.push(thread::spawn(move || connect_db(connect)));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        } else {
            for _ in 0..num_connections {
                connect_db(connect);
            }
        }
        print_memory_usage();
    }
}
