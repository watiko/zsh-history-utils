use std::process::Command;

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn git_version() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--always", "--dirty=-modified"])
        .output()
        .ok()?;
    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

fn version() -> String {
    format!(
        "{}-{}",
        PACKAGE_VERSION,
        git_version().unwrap_or_else(|| "unknown".to_owned())
    )
}

fn main() {
    // track changes of git
    println!("cargo:rerun-if-changed=.git");

    println!("cargo::rustc-env=VERSION={}", version());
}
