use regex::Regex;
use sysinfo::{ProcessExt, SystemExt};

// TODO: send KILLSIG to process or list of processes

fn main() {
    let proc_name = std::env::args().nth(1).expect("no process name given");
    println!("looking for : {}", proc_name);

    let mut system = sysinfo::System::new_all();
    let matcher = format!(r"{}$", proc_name);
    let re = Regex::new(&matcher).unwrap();
    system.refresh_all();

    for (pid_, proc_) in system.get_processes() {
        // TODO: add guard for process status
        if re.is_match(&proc_.name()) {
            println!("{}:{} => status {:?}", pid_, proc_.name(), proc_.status())
        }
    }
}
