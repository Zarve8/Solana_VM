/// Return the remaining compute units the program may consume
#[inline]
pub fn sol_remaining_compute_units() -> u64 {
    #[cfg(feature = "solvm")]
    {
        crate::custom::sol_remaining_compute_units()
    }
}
