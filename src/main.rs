use sysinfo::{ProcessExt, SystemExt};

// TODO: find process in list
// TODO: send KILLSIG to process or list of processes

fn main() {
    let pname = std::env::args().nth(1).expect("no process name given");
    println!("{}", pname);

    let mut system = sysinfo::System::new_all();

    system.refresh_all();

    for (pid, proc_) in system.get_processes() {
        // TODO: add guard for process status
        println!("{}:{} => status {:?}", pid, proc_.name(), proc_.status());
    }
}
