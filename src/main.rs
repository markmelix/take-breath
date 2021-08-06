mod lib;

use lib::*;
use std::time::Duration;

fn main() {
    let work = Work::new(
	Duration::from_secs(45 * 60), // Work duration: 45 minutes
	Duration::from_secs(2 * 60), // Idle while work min duration: 2 minutes
    );

    let rest = Rest::new(
	Duration::from_secs(15 * 60) // Rest duration: 15 minutes
    );

    loop {
	work.count();
	rest.count();
    }
}
