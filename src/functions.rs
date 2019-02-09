pub mod factorial {
    use num_bigint::{ToBigInt, BigInt};

    pub fn bigint_factorial(i: u32) -> BigInt {
        (1..=i).into_iter()
        .map(|x| bigint(x))
        .fold(bigint(1), |product, val| product * val)
    }

    fn bigint(i: u32) -> BigInt {
        i.to_bigint().unwrap()
    }

}
