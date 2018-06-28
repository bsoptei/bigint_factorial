pub mod factorial {
    extern crate num_bigint;
    use self::num_bigint::{ToBigInt, BigInt};

    pub fn bigint_factorial(i: u32) -> String {
        (1..=i).into_iter()
        .map(|x| bigint(x))
        .fold(bigint(1), |product, val| product * val)
        .to_str_radix(10)
    }

    fn bigint(i: u32) -> BigInt {
        i.to_bigint().unwrap()
    }

}
