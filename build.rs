/// This build.rs script is used to generate build-time information and set environment variables for the build process.
/// It retrieves the Rust compiler version and sets it as the `RUSTC_VERSION` environment variable.
/// It also sets the `BUILD_VERSION` environment variable to the value of `NEARCORE_VERSION` defined in the project.
/// Additionally, it prints messages to indicate which files should trigger a rebuild when changed.
fn get_rustc_version() -> anyhow::Result<String> {
    let version = rustc_version::version()?;
    Ok(version.to_string())
}

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");

    println!("cargo:rustc-env=BUILD_VERSION={}", env!("NEARCORE_VERSION"));

    let rustc_version = get_rustc_version()?;
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);

    Ok(())
}
