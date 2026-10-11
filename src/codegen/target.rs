#[allow(nonstandard_style)]
#[derive(Debug, PartialEq)]
/// Target of code generation.
/// Taken from object-lang documentation: https://crates.io/crates/object-lang
pub enum CodeGenTarget {
    Linux_x86_64,
    Linux_aarch64,
    macOS_x86_64,
    macOS_Apple_Silicon,
    Windows_x86_64,
}
