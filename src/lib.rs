mod functions;

#[cfg(test)]
mod tests {

    use functions::factorial::*;

    #[test]
    fn factorial_2000() {
        let result = bigint_factorial(2000);
        let first_32_digits = &result[..32];

        assert_eq!(
            "33162750924506332411753933805763",
            first_32_digits
        );

        assert_eq!(
            result.len(),
            5736
        );
    }
}
