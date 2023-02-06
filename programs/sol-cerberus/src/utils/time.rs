use anchor_lang::prelude::*;

pub fn utc_now() -> i64 {
    // Total number of seconds since the Unix epoch.
    Clock::get().unwrap().unix_timestamp
}
