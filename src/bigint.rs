pub mod num_bigint {
    use num_bigint::BigUint;

    /// Returns an iterator of the base-10 digits of num
    pub fn digits(num: &BigUint) -> impl Iterator<Item = u128> {
        num.to_radix_be(10)
            .into_iter()
            .map(|d| <u8 as Into<u128>>::into(d))
    }
}
