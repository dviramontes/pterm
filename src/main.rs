use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use regex::Regex;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let proc_name = std::env::args().nth(1).expect("no process name given");
    let mut system = sysinfo::System::new_all();
    let matcher = format!(r"(?i){}$", proc_name);
    let re = Regex::new(&matcher).unwrap();

    system.refresh_all();

    for (pid, proc) in system.get_processes() {
        if re.is_match(&proc.name()) {
            match proc.status() {
                sysinfo::ProcessStatus::Run => {
                    signal::kill(Pid::from_raw(*pid), Signal::SIGTERM).unwrap();
                    println!("{} {} SIGTERM", pid, proc.name())
                }
                _ => println!("process: {} not currently running", proc_name),
            }
        }
    }
}
