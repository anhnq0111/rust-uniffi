uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}
