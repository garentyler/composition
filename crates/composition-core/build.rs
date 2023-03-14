use std::process::Command;
fn main() {
    if let Ok(output) = Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        let git_hash = String::from_utf8_lossy(&output.stdout).to_string();
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    } else {
        println!("cargo:rustc-env=GIT_HASH=00000000");
    }
}
