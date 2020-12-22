mod compute;
mod models;
mod parser;

pub fn parse_and_compute(str_calculation: String, verbosity: u8) -> Result<f64, models::ErrorList> {
    if verbosity > 4 {
        println!("Start parse_and_compute: {:?}", str_calculation);
    }
    return match parser::parse_operation(&str_calculation, verbosity) {
        Ok(operation) => compute::compute(&operation, verbosity),
        Err(err) => Err(err),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_ok {
        ($calculation: expr, $result: literal) => {
            let expected: Result<f64, models::ErrorList> = Ok($result);
            assert_eq!(parse_and_compute($calculation.to_string(), 5), expected);
        };
    }

    #[test]
    fn it_should_handle_explicit_positive_value() {
        assert_ok!("+2", 2.0);
        assert_ok!("+2.0", 2.0);
        assert_ok!("+2.1", 2.1);
    }

    #[test]
    fn it_should_handle_negative_value() {
        assert_ok!("-2", -2.0);
        assert_ok!("-2.0", -2.0);
        assert_ok!("-2.1", -2.1);
    }

    #[test]
    fn it_should_handle_operation_with_negatives_value() {
        assert_ok!("-2+1", -1.0);
        assert_ok!("-2*6", -12.0);
        assert_ok!("6*-2", -12.0);
        assert_ok!("1+6*-2", -11.0);
        assert_ok!("6*-2+1", -11.0);
        assert_ok!("6*-2-3", -15.0);
        assert_ok!("-3+6*-2", -15.0);
    }

    #[test]
    fn it_should_handle_one_operator_add() {
        assert_ok!("3+2", 5.0);
    }

    #[test]
    fn it_should_handle_one_operator_minus() {
        assert_ok!("3-2", 1.0);
    }

    #[test]
    fn it_should_handle_one_operator_multiply() {
        assert_ok!("3*2", 6.0);
    }

    #[test]
    fn it_should_handle_one_operator_divide() {
        assert_ok!("3/2", 1.5);
    }

    #[test]
    fn it_should_handle_multiple_add_operator() {
        assert_ok!("4+3+2+1", 10.0);
    }

    #[test]
    fn it_should_handle_mix_add_minus_operator() {
        assert_ok!("4-3+2-1", 2.0);
    }

    #[test]
    fn it_should_handle_add_multiply_operator_priority() {
        assert_ok!("4+3*2", 10.0);
        assert_ok!("4+3*2+1", 11.0);
    }

    #[test]
    fn it_should_handle_add_divide_operator_priority() {
        assert_ok!("4+3/2+1", 6.5);
    }

    #[test]
    fn it_should_handle_float_number() {
        assert_ok!("3.0+2.0", 5.0);
        assert_ok!("4.0+3.0+2.0-1.0", 8.0);
        assert_ok!("4.0+3.0/2.0+1.0", 6.5);
        assert_ok!("4.2*1.3", 5.460000000000001);
    }

    #[test]
    fn it_should_ignore_invalid_characters() {
        assert_ok!("        3 + 2   ", 5.0);
        assert_ok!("        3.0 + 2.0   ", 5.0);
        assert_ok!("4 +3 +2+1", 10.0);
        assert_ok!("4+3/ 2+ 1", 6.5);
    }

    #[test]
    fn it_should_handle_simple_factorial() {
        assert_ok!("1!", 1.0);
        assert_ok!("2!", 2.0);
        assert_ok!("3!", 6.0);
        assert_ok!("4!", 24.0);
        assert_ok!("5!", 120.0);
        assert_ok!("6!", 720.0);
        assert_ok!("7!", 5040.0);
    }

    #[test]
    fn it_should_handle_multiple_factorial() {
        assert_ok!("1!!", 1.0);
        assert_ok!("2!!", 2.0);
        assert_ok!("3!!", 720.0);
    }

    #[test]
    fn it_should_handle_factorial_with_other_operations() {
        assert_ok!("1+1!", 2.0);
        assert_ok!("1+2!", 3.0);
        assert_ok!("1+3!", 7.0);
        assert_ok!("2*1!", 2.0);
        assert_ok!("2*2!", 4.0);
        assert_ok!("2*3!", 12.0);
    }

    #[test]
    fn it_should_handle_factorial_combine_with_multiplication_permutability() {
        assert_ok!("2*1!", 2.0);
        assert_ok!("1!*2", 2.0);
        assert_ok!("2*2!", 4.0);
        assert_ok!("2!*2", 4.0);
        assert_ok!("2*3!", 12.0);
        assert_ok!("3!*2", 12.0);
    }

    #[test]
    fn it_should_handle_factorial_combine_with_addition_permutability() {
        assert_ok!("1+1!", 2.0);
        assert_ok!("1!+1", 2.0);
        assert_ok!("1+2!", 3.0);
        assert_ok!("2!+1", 3.0);
        assert_ok!("1+3!", 7.0);
        assert_ok!("3!+1", 7.0);
    }

    #[test]
    fn it_should_handle_factorial_combine_with_addition_and_multiplication_permutability() {
        assert_ok!("1+2!*2", 5.0);
        assert_ok!("1+2*2!", 5.0);
        assert_ok!("2*2!+1", 5.0);
        assert_ok!("2!*2+1", 5.0);
    }
}
