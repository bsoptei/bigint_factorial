pub mod factorial {
    extern crate num_bigint;
    use self::num_bigint::{ToBigInt, BigInt};

    pub fn bigint_factorial(i: i32) -> String {
        bigint_range(1, i, true).into_iter()
        .fold( bigint(1), |product, val| product * val)
        .to_str_radix(10)
    }

    fn bigint(i: i32) -> BigInt {
        i.to_bigint().unwrap()
    }

    fn bigint_range(from: i32, to: i32, inclusive: bool) -> Vec<BigInt> {
        let corr = if inclusive { 1 } else { 0 };
        (from..to + corr).into_iter()
        .map(|x| bigint(x)).collect()
    }

}
