use notify_rust::{Notification, Timeout};
use tokio::time::{self, Duration};
use user_idle::UserIdle;

#[tokio::main]
async fn main() {
    let mut work_ctr = 0;
    let mut rest_ctr = 0;

    loop {
	work_trigger();

	while work_ctr < 45 * 60 {
	    wait(1).await;
	    work_ctr += 1;
	}

	rest_trigger();

	while rest_ctr < 15 * 60 {
	    if idle_time() > 0 {
		wait(1).await;
		rest_ctr += 1;
	    } else {
		short_rest_trigger();
		wait(2).await;
	    }
	}

	work_ctr = 0;
	rest_ctr = 0;
    }
}

async fn wait(secs: u64) {
    time::sleep(Duration::from_secs(secs)).await;
}

fn rest_trigger() {
    Notification::new()
	.summary("Take a breath!")
	.body("It's time to take a breath.")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn work_trigger() {
    Notification::new()
	.summary("Work!")
	.body("It's time to work.")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn short_rest_trigger() {
    Notification::new()
	.summary("Take a breath more!")
	.body("You had too little rest!")
	.timeout(Timeout::Milliseconds(5000))
	.show()
	.unwrap();
}

fn idle_time() -> u64 {
    UserIdle::get_time().unwrap().as_seconds()
}
