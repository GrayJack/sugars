//! Module for time related macros

/// Creates a `Duration` object following a time pattern
///
/// **Paterns:**
/// * min: minutes
/// * sec: seconds
/// * nano: nanoseconds
/// * micro: microseconds
/// * milli: milliseconds
///
/// # Examples
/// ```rust
/// use sugars::dur;
///
/// # fn main() {
/// let d = dur!(10 sec);
/// // Sleeps for 10 seconds
/// std::thread::sleep(d);
/// # }
/// ```
#[macro_export]
macro_rules! dur {
    ($e:literal min) => {{
        let min2sec = $e * 60;
        std::time::Duration::from_secs(min2sec)
    }};
    ($i:ident min) => {{
        let min2sec = $i * 60;
        std::time::Duration::from_secs(min2sec)
    }};

    ($e:literal sec) => {
        std::time::Duration::from_secs($e)
    };
    ($i:ident sec) => {
        std::time::Duration::from_secs($i)
    };

    ($e:literal nano) => {
        std::time::Duration::from_nanos($e)
    };
    ($i:ident nano) => {
        std::time::Duration::from_nanos($i)
    };

    ($e:literal micro) => {
        std::time::Duration::from_micros($e)
    };
    ($i:ident micro) => {
        std::time::Duration::from_micros($i)
    };

    ($e:literal milli) => {
        std::time::Duration::from_millis($e)
    };
    ($i:ident milli) => {
        std::time::Duration::from_millis($i)
    };
}

/// Makes a thread sleep a amount following a time pattern
///
/// **Paterns:**
/// * min: minutes
/// * sec: seconds
/// * nano: nanoseconds
/// * micro: microseconds
/// * milli: milliseconds
///
/// # Examples
/// ```rust
/// use sugars::sleep;
/// # fn main() {
/// // Thread sleeps for 10 seconds
/// sleep!(10 sec);
/// # }
/// ```
#[macro_export]
macro_rules! sleep {
    ($e:literal min) => {{
        let min2sec = $e * 60;
        let dur = std::time::Duration::from_secs(min2sec);
        std::thread::sleep(dur);
    }};
    ($i:ident min) => {{
        let min2sec = $i * 60;
        let dur = std::time::Duration::from_secs(min2sec);
        std::thread::sleep(dur);
    }};

    ($e:literal sec) => {{
        let dur = std::time::Duration::from_secs($e);
        std::thread::sleep(dur);
    }};
    ($i:ident sec) => {{
        let dur = std::time::Duration::from_secs($i);
        std::thread::sleep(dur);
    }};

    ($e:literal nano) => {{
        let dur = std::time::Duration::from_nanos($e);
        std::thread::sleep(dur);
    }};
    ($i:ident nano) => {{
        let dur = std::time::Duration::from_nanos($i);
        std::thread::sleep(dur);
    }};

    ($e:literal micro) => {{
        let dur = std::time::Duration::from_micros($e);
        std::thread::sleep(dur);
    }};
    ($i:ident micro) => {{
        let dur = std::time::Duration::from_micros($i);
        std::thread::sleep(dur);
    }};

    ($e:literal milli) => {{
        let dur = std::time::Duration::from_millis($e);
        std::thread::sleep(dur);
    }};
    ($i:ident milli) => {{
        let dur = std::time::Duration::from_millis($i);
        std::thread::sleep(dur);
    }};
}

/// Print out the time it took to execute a given expression in seconds.
///
/// Much like Rust standard library dbg! macro, but prints the
/// time it takes to run what is inside.
///
/// **Note:** requires Rust unstable feature `duration_float`,
/// so it lives on crate feature `nightly`.
///
/// # Example
/// ```rust
/// # #![feature(duration_float)]
/// use sugars::{time, dur};
/// # fn main() {
/// let d = dur!(10 sec);
/// // Sleeps for 10 seconds
/// time!(std::thread::sleep(d)); // Should print 10.000 at least
/// # }
/// ```
#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! time {
    ($e:expr) => {{
        use std::time::Instant;
        let time = Instant::now();
        match $e {
            tmp => {
                eprintln!("{:.6} seconds", time.elapsed().as_secs_f64());
                tmp
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn dur_literal_min() {
        let expected = Duration::from_secs(10 * 60);
        let test = dur!(10 min);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_identifier_min() {
        let expected = Duration::from_secs(10 * 60);
        let x = 10;
        let test = dur!(x min);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_literal_sec() {
        let expected = Duration::from_secs(10);
        let test = dur!(10 sec);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_identifier_sec() {
        let expected = Duration::from_secs(10);
        let x = 10;
        let test = dur!(x sec);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_literal_nano() {
        let expected = Duration::from_nanos(10);
        let test = dur!(10 nano);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_identifier_nano() {
        let expected = Duration::from_nanos(10);
        let x = 10;
        let test = dur!(x nano);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_literal_micros() {
        let expected = Duration::from_micros(10);
        let test = dur!(10 micro);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_identifier_micros() {
        let expected = Duration::from_micros(10);
        let x = 10;
        let test = dur!(x micro);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_literal_millis() {
        let expected = Duration::from_millis(10);
        let test = dur!(10 milli);

        assert_eq!(expected, test);
    }

    #[test]
    fn dur_identifier_millis() {
        let expected = Duration::from_millis(10);
        let x = 10;
        let test = dur!(x milli);

        assert_eq!(expected, test);
    }
}
