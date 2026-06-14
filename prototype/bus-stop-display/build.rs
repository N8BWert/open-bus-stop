//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use chrono::{Datelike, Local, Timelike, Weekday};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let now = Local::now() + chrono::Duration::seconds(30);
    println!("cargo:rustc-env=BUILD_YEAR={}", now.year());
    println!("cargo:rustc-env=BUILD_MONTH={}", now.month());
    println!("cargo:rustc-env=BUILD_DAY={}", now.day());
    println!("cargo:rustc-env=BUILD_HOUR={}", now.hour());
    println!("cargo:rustc-env=BUILD_MINUTE={}", now.minute());
    println!("cargo:rustc-env=BUILD_SECOND={}", now.second());
    match now.weekday() {
        Weekday::Mon => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=1"),
        Weekday::Tue => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=2"),
        Weekday::Wed => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=3"),
        Weekday::Thu => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=4"),
        Weekday::Fri => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=5"),
        Weekday::Sat => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=6"),
        Weekday::Sun => println!("cargo:rustc-env=BUILD_DAY_OF_WEEK=7"),
    }

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=PATH");
}
