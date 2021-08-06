use notify_rust::{Notification, Timeout};
use user_idle::UserIdle;
use std::time::Duration;
use std::thread;

fn main() {
    loop {
	work_trigger();
	work_counter();
	rest_trigger();
	rest_counter();
    }
}

fn work_counter() {
    let mut ctr = 0;
    while ctr < 45 * 60 {
	if idle_time() < 60 {
	    wait(1);
	    ctr += 1;
	} else {
	    idle_while_work_trigger();
	    loop {
		if idle_time() == 0 {
		    work_resumed_trigger();
		    break;
		}
	    }
	}
	println!("{} ", ctr);
    }
}

fn rest_counter() {
    for ctr in 0..(15 * 60) {
	if idle_time() > 0 {
	    wait(1);
	} else {
	    short_rest_trigger();
	    wait(2);
	}
    }
}

fn wait(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

fn rest_trigger() {
    Notification::new()
	.summary("Take a breath")
	.body("It's time to take a breath.")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn work_trigger() {
    Notification::new()
	.summary("Take a breath")
	.body("It's time to work.")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn short_rest_trigger() {
    Notification::new()
	.summary("Take a breath")
	.body("You had too little rest!")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn idle_while_work_trigger() {
    Notification::new()
	.summary("Take a breath")
	.body("Idle while work counter started")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn work_resumed_trigger() {
    Notification::new()
	.summary("Take a breath")
	.body("Work has been resumed")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn idle_time() -> u64 {
    UserIdle::get_time().unwrap().as_seconds()
}
