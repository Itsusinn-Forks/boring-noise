#[cfg(feature = "ring")]
pub use ring::*;

#[cfg(feature = "ring")]
pub fn verify_slices_are_equal(a: &[u8], b: &[u8]) -> Result<(), error::Unspecified> {
    if !constant_time_eq::constant_time_eq(a, b) {
        return Err(ring::error::Unspecified);
    }
    Ok(())
}

#[cfg(feature = "aws-lc-rs")]
pub use aws_lc_rs::*;

#[cfg(feature = "aws-lc-rs")]
pub use aws_lc_rs::constant_time::verify_slices_are_equal;
