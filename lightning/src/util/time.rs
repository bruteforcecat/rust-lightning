// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

//! A very simple serialization framework which is used to serialize/deserialize messages as well
//! as ChannelsManagers and ChannelMonitors.

use core::ops::Sub;
use core::time::Duration;
/// A measurement of time.
pub trait Time: Copy + Sub<Duration, Output = Self> where Self: Sized {
	/// Returns an instance corresponding to the current moment.
	fn now() -> Self;

	/// Returns the amount of time elapsed since `self` was created.
	fn elapsed(&self) -> Duration;

	/// Returns the amount of time passed between `earlier` and `self`.
	fn duration_since(&self, earlier: Self) -> Duration;

	/// Returns the amount of time passed since the beginning of [`Time`].
	///
	/// Used during (de-)serialization.
	fn duration_since_epoch() -> Duration;
}

/// A state in which time has no meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Eternity;

impl Time for Eternity {
	fn now() -> Self {
		Self
	}
	
	fn duration_since(&self, _earlier: Self) -> Duration {
		Duration::from_secs(0)
	}
	
	fn duration_since_epoch() -> Duration {
		Duration::from_secs(0)
	}
	
	fn elapsed(&self) -> Duration {
		Duration::from_secs(0)
	}
}

impl Sub<Duration> for Eternity {
	type Output = Self;

	fn sub(self, _other: Duration) -> Self {
		self
	}
}

#[cfg(not(feature = "no-std"))]
impl Time for std::time::Instant {
	fn now() -> Self {
		std::time::Instant::now()
	}
	
	fn duration_since(&self, earlier: Self) -> Duration {
		self.duration_since(earlier)
	}
	
	fn duration_since_epoch() -> Duration {
		use std::time::SystemTime;
		SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
	}
	fn elapsed(&self) -> Duration {
		std::time::Instant::elapsed(self)
	}
}

#[cfg(not(feature = "no-std"))]
impl Time for std::time::SystemTime {
	fn now() -> Self {
		std::time::SystemTime::now()
	}

	fn duration_since(&self, earlier: Self) -> Duration {
		self.duration_since(earlier).unwrap()
	}

	fn duration_since_epoch() -> Duration {
		use std::time::SystemTime;
		SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
	}

	fn elapsed(&self) -> Duration {
		std::time::SystemTime::elapsed(self).unwrap()
	}
}



#[cfg(test)]
pub mod tests {
	use util::time::{Time, Eternity};
	use util::test_utils::SinceEpoch;

	use core::time::Duration;

	#[test]
	fn time_passes_when_advanced() {
		let now = SinceEpoch::now();
		assert_eq!(now.elapsed(), Duration::from_secs(0));

		SinceEpoch::advance(Duration::from_secs(1));
		SinceEpoch::advance(Duration::from_secs(1));

		let elapsed = now.elapsed();
		let later = SinceEpoch::now();

		assert_eq!(elapsed, Duration::from_secs(2));
		assert_eq!(later - elapsed, now);
	}

	#[test]
	fn time_never_passes_in_an_eternity() {
		let now = Eternity::now();
		let elapsed = now.elapsed();
		let later = Eternity::now();

		assert_eq!(now.elapsed(), Duration::from_secs(0));
		assert_eq!(later - elapsed, now);
	}
}
