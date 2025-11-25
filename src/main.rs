

mod cpu_fast;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <suffix> [case-sensitive]", args[0]);
        std::process::exit(1);
    }

    let suffix = &args[1];
    let case_sensitive = args.len() >= 3 && args[2].eq_ignore_ascii_case("true");

    // Always run the fast CPU version
    cpu_fast::run_cpu_fast(suffix, case_sensitive);
}