use std::time::Duration;
pub const DEFAULT_WORK_DUR: Duration = Duration::from_secs(45 * 60);
pub const DEFAULT_WORK_IDLE_TO_PAUSE: Duration = Duration::from_secs(2 * 60);
pub const DEFAULT_REST_DUR: Duration = Duration::from_secs(15 * 60);

#[cfg(feature = "config")]
pub mod config {
    use super::*;
    use toml;
    use std::path::Path;
    use serde::{Serialize, Deserialize};
    use std::time::Duration;
    use std::error::Error;
    use std::fs;

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Config {
	#[serde(default)]
	pub work_time: WorkTime,

	#[serde(default)]
	pub rest_time: RestTime,
    }

    impl Config {
	pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Box<dyn Error>> {
	    let file_data = fs::read_to_string(path)?;
	    Ok(toml::from_str(&file_data)?)
	}

	pub fn apply_from_file<T: AsRef<Path>>(path: T) -> Self {
	    match Self::from_file(path) {
		Ok(config) => config,
		Err(_) => Self::default(),
	    }
	}
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WorkTime {
	#[serde(with = "humantime_serde")]
	#[serde(default = "WorkTime::default_duration")]
	pub duration: Duration,

	#[serde(with = "humantime_serde")]
	#[serde(default = "WorkTime::default_idle_to_pause")]
	pub idle_to_pause: Duration,
    }

    impl WorkTime {
	fn default_duration() -> Duration {
	    DEFAULT_WORK_DUR
	}

	fn default_idle_to_pause() -> Duration {
	    DEFAULT_WORK_IDLE_TO_PAUSE
	}
    }

    impl Default for WorkTime {
	fn default() -> Self {
	    Self {
		duration: Self::default_duration(),
		idle_to_pause: Self::default_idle_to_pause(),
	    }
	}
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RestTime {
	#[serde(with = "humantime_serde")]
	#[serde(default = "RestTime::default_duration")]
	pub duration: Duration,
    }

    impl RestTime {
	fn default_duration() -> Duration {
	    DEFAULT_REST_DUR
	}
    }

    impl Default for RestTime {
	fn default() -> Self {
	    Self {
		duration: Self::default_duration(),
	    }
	}
    }
}

pub mod counter {
    #[cfg(feature = "notify")]
    use notify_rust::{Notification, Timeout};
    use user_idle::UserIdle;
    use std::time::Duration;
    use std::thread;

    /// Work time counter structure.
    pub struct Work {
	/// Work duration.
	work_dur: Duration,

	/// How much time computer have to be idle to disable work time counting.
	idle_dur: Duration,
    }

    impl Work {
	/// Returns a new work time counter.
	pub fn new(work_dur: Duration,
		   idle_dur: Duration) -> Self {
	    Self {
		work_dur,
		idle_dur,
	    }
	}

	/// Starts a work time counter.
	pub fn count(&self) {
	    let mut ctr = 0;
	    let idle_dur = self.idle_dur.as_secs();
	    let work_dur = self.work_dur.as_secs();

	    self.trigger();
	    while ctr < work_dur {
		if idle_time() < idle_dur {
		    wait(1);
		    ctr += 1;
		} else {
		    if (ctr as i32 - idle_dur as i32) <= 0 {
			ctr = 0;
		    } else {
			ctr -= idle_dur;
		    }

		    self.idle_trigger();
		    loop {
			if idle_time() == 0 {
			    self.work_resumed_trigger();
			    break;
			}
		    }
		}
	    }
	}

	/// Function to run when it is time to work.
	fn trigger(&self) {
	    println!("It's time to work.");
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath: Work")
		.body("It's time to work.")
		.timeout(Timeout::Milliseconds(5000))
		.show()
		.unwrap();
	}

	/// Function to run when idle while work.
	fn idle_trigger(&self) {
	    println!("Idle while work counter started.");
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath: Work Idle")
		.body("Idle while work counter started.")
		.timeout(Timeout::Milliseconds(5000))
		.show()
		.unwrap();
	}

	/// Function to run when work has been resumed.
	fn work_resumed_trigger(&self) {
	    println!("Work has been resumed.");
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath: Work Resumed")
		.body("Work has been resumed.")
		.timeout(Timeout::Milliseconds(5000))
		.show()
		.unwrap();
	}
    }

    /// Rest time counter structure.
    pub struct Rest {
	/// Rest duration.
	rest_dur: Duration,
    }

    impl Rest {
	/// Returns a new rest time counter.
	pub fn new(rest_dur: Duration) -> Self {
	    Self {
		rest_dur,
	    }
	}

	/// Starts a rest time counter.
	pub fn count(&self) {
	    let mut ctr = 0;
	    let rest_dur = self.rest_dur.as_secs();

	    self.trigger();
	    while ctr < rest_dur {
		wait(1);
		if idle_time() > 0 {
		    ctr += 1;
		} else {
		    self.short_rest_trigger(Duration::from_secs(ctr));
		    loop {
			if idle_time() > 0 {
			    self.rest_resumed_trigger();
			    break;
			}
		    }
		}
	    }
	}

	/// Function to run when it is time to take a breath.
	fn trigger(&self) {
	    println!("It's time ti take a breath.");
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath")
		.body("It's time to take a breath.")
		.timeout(Timeout::Milliseconds(5000))
		.show()
		.unwrap();
	}

	/// Function to run when the rest is too short.
	fn short_rest_trigger(&self, ctr: Duration) {
	    let left = (self.rest_dur - ctr).as_secs() as f32 / 60.0;
	    println!("You had too little rest! {:.2} minutes left.", left);
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath")
		.body(&format!("You had too little rest!\n{:.2} minutes left.", left))
		.timeout(Timeout::Milliseconds(10000))
		.show()
		.unwrap();
	}

	/// Function to run when rest has been resumed.
	fn rest_resumed_trigger(&self) {
	    println!("Rest has been resumed.");
	    #[cfg(feature = "notify")]
	    Notification::new()
		.summary("Take a breath")
		.body("Rest has been resumed.")
		.timeout(Timeout::Milliseconds(5000))
		.show()
		.unwrap();
	}
    }

    /// Wait some time in seconds.
    fn wait(secs: u64) {
	thread::sleep(Duration::from_secs(secs));
    }

    /// Get computer idle time.
    fn idle_time() -> u64 {
	UserIdle::get_time().unwrap().as_seconds()
    }
}
