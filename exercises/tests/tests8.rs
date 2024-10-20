// tests8.rs
//
// This exercise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    println!("cargo:rustc-cfg=pass");
}