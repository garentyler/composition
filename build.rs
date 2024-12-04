use std::process::Command;
fn main() {
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        let git_hash = String::from_utf8_lossy(&output.stdout).to_string();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    } else {
        println!("cargo:rustc-env=GIT_HASH=000000000");
    }

    if let Ok(output) = Command::new("git")
        .args(["log", "-1", "--format=%cI"])
        .output()
    {
        let iso_date = String::from_utf8_lossy(&output.stdout).to_string();
        println!("cargo:rustc-env=GIT_DATE={}", iso_date);
    } else {
        println!("cargo:rustc-env=GIT_DATE=1970-01-01");
    }

    println!(
        "cargo:rustc-env=BUILD_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}
