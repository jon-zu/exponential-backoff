extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::{fs, thread, time::Duration};

#[cfg(feature = "std")]
#[test]
fn doesnt_crash() -> std::io::Result<()> {
    let retries = 8;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let backoff = Backoff::new(retries, min, max);

    for duration in &backoff {
        println!("duration {:?}", duration);
        match fs::read_to_string("README.md") {
            Ok(_string) => return Ok(()),
            Err(_) => thread::sleep(duration),
        }
    }

    unreachable!();
}

#[cfg(feature = "std")]
#[test]
fn iterator_completes() {
    let retries = 3;
    let min = Duration::from_millis(10);
    let max = Duration::from_millis(20);
    let backoff = Backoff::new(retries, min, max);
    let mut counter = 0;
    for duration in &backoff {
        counter += 1;
        thread::sleep(duration);
    }
    assert_eq!(counter, 1 + retries);
}

#[cfg(feature = "std")]
#[test]
fn max_backoff_without_crashing() {
    let retries = u32::MAX;
    let min = Duration::MAX;
    let backoff = Backoff::new(retries, min, None);

    let mut counter = 0u32;
    for _ in &backoff {
        counter += 1;
        if counter > 32 {
            break;
        }
    }
}

