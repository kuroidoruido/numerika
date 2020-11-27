use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
    Factorial,
}

fn operator_from_char(char_op: char) -> Option<Operator> {
    match char_op {
        '+' => Some(Operator::Add),
        '-' => Some(Operator::Minus),
        '*' => Some(Operator::Multiply),
        '/' => Some(Operator::Divide),
        '!' => Some(Operator::Factorial),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorList {
    Unknown,
    CannotParseNumber(String),
    InvalidOperatorPosition,
    ThisOperatorShouldBeAlreadyComputed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperationUnit {
    Operand(f64),
    Operator(Operator),
}

pub fn parse_and_compute(str_calculation: String, verbosity: u8) -> Result<f64, ErrorList> {
    if verbosity > 4 {
        println!("Start parse_and_compute: {:?}", str_calculation);
    }
    return match parse_operation(&str_calculation, verbosity) {
        Ok(operation) => compute(&operation, verbosity),
        Err(err) => Err(err),
    };
}

fn parse_operation(
    str_calculation: &String,
    verbosity: u8,
) -> Result<VecDeque<OperationUnit>, ErrorList> {
    let mut calc_iter = str_calculation.chars();
    let mut operation: VecDeque<OperationUnit> = VecDeque::new();
    let mut current_number: String = String::from("");
    loop {
        match calc_iter.next() {
            Some(n @ '1') | Some(n @ '2') | Some(n @ '3') | Some(n @ '4') | Some(n @ '5')
            | Some(n @ '6') | Some(n @ '7') | Some(n @ '8') | Some(n @ '9') | Some(n @ '0')
            | Some(n @ '.') => current_number.push(n),
            Some(op @ '+') | Some(op @ '-') | Some(op @ '*') | Some(op @ '/') | Some(op @ '!') => {
                if !current_number.is_empty() {
                    match parse_number(&current_number) {
                        Ok(n) => {
                            operation.push_back(OperationUnit::Operand(n));
                            current_number.clear();
                        }
                        Err(err) => return Err(err),
                    }
                }
                operation.push_back(OperationUnit::Operator(operator_from_char(op).unwrap()));
            }
            Some(other @ _) => {
                if verbosity > 1 {
                    println!("Ignore {:?}", other);
                }
            }
            None => {
                if !current_number.is_empty() {
                    match parse_number(&current_number) {
                        Ok(n) => {
                            operation.push_back(OperationUnit::Operand(n));
                            current_number.clear();
                        }
                        Err(err) => return Err(err),
                    }
                }
                return Ok(operation);
            }
        }
    }
}

fn parse_number(current_number: &String) -> Result<f64, ErrorList> {
    return match current_number.parse::<f64>() {
        Err(_) => Err(ErrorList::CannotParseNumber(current_number.clone())),
        Ok(n) => Ok(n),
    };
}

fn compute(operation: &VecDeque<OperationUnit>, verbosity: u8) -> Result<f64, ErrorList> {
    if verbosity > 1 {
        println!("compute operation: {:?}", operation);
    }
    return compute_unary(&operation, verbosity);
}

fn compute_unary(operation: &VecDeque<OperationUnit>, verbosity: u8) -> Result<f64, ErrorList> {
    if verbosity > 1 {
        println!("compute_unary operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<OperationUnit> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(OperationUnit::Operator(op)) => match op {
                Operator::Factorial => match result_stack.pop_back() {
                    Some(OperationUnit::Operator(_)) => {
                        return Err(ErrorList::InvalidOperatorPosition)
                    }
                    Some(OperationUnit::Operand(n)) => {
                        result_stack.push_back(OperationUnit::Operand(factorial(n, verbosity)));
                        operation_unit_index += 1;
                    }
                    None => return Err(ErrorList::InvalidOperatorPosition),
                },
                Operator::Add => match result_stack.back() {
                    Some(OperationUnit::Operand(_)) => {
                        result_stack.push_back(OperationUnit::Operator(Operator::Add));
                        operation_unit_index += 1;
                    }
                    _ => {
                        operation_unit_index += 1;
                        // nothing to do we can ignore it because the case is one of these: "+2", "2++2", "2*+2", ...
                    }
                },
                Operator::Minus => match result_stack.back() {
                    Some(OperationUnit::Operand(_)) => {
                        result_stack.push_back(OperationUnit::Operator(Operator::Minus));
                        operation_unit_index += 1;
                    }
                    _ => match operation.get(operation_unit_index + 1) {
                        Some(OperationUnit::Operand(n)) => {
                            result_stack.push_back(OperationUnit::Operand(-n));
                            operation_unit_index += 2;
                        }
                        _ => return Err(ErrorList::InvalidOperatorPosition),
                    },
                },
                op => {
                    result_stack.push_back(OperationUnit::Operator(*op));
                    operation_unit_index += 1;
                }
            },
            Some(n @ OperationUnit::Operand(_)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => return compute_multiply_divide(&result_stack, verbosity),
        }
    }
}

fn compute_multiply_divide(
    operation: &VecDeque<OperationUnit>,
    verbosity: u8,
) -> Result<f64, ErrorList> {
    if verbosity > 1 {
        println!("compute_multiply_divide operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<OperationUnit> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(OperationUnit::Operator(op)) => match op {
                Operator::Multiply => match result_stack.pop_back() {
                    Some(OperationUnit::Operator(_)) => {
                        return Err(ErrorList::InvalidOperatorPosition)
                    }
                    Some(OperationUnit::Operand(last_operand)) => {
                        match operation.get(operation_unit_index + 1) {
                            Some(OperationUnit::Operator(_)) => {
                                return Err(ErrorList::InvalidOperatorPosition)
                            }
                            Some(OperationUnit::Operand(next_operand)) => {
                                result_stack
                                    .push_back(OperationUnit::Operand(last_operand * next_operand));
                                operation_unit_index += 2;
                            }
                            None => return Err(ErrorList::InvalidOperatorPosition),
                        }
                    }
                    None => return Err(ErrorList::InvalidOperatorPosition),
                },
                Operator::Divide => match result_stack.pop_back() {
                    Some(OperationUnit::Operator(_)) => {
                        return Err(ErrorList::InvalidOperatorPosition)
                    }
                    Some(OperationUnit::Operand(last_operand)) => {
                        match operation.get(operation_unit_index + 1) {
                            Some(OperationUnit::Operator(_)) => {
                                return Err(ErrorList::InvalidOperatorPosition)
                            }
                            Some(OperationUnit::Operand(next_operand)) => {
                                result_stack
                                    .push_back(OperationUnit::Operand(last_operand / next_operand));
                                operation_unit_index += 2;
                            }
                            None => return Err(ErrorList::InvalidOperatorPosition),
                        }
                    }
                    None => return Err(ErrorList::InvalidOperatorPosition),
                },
                op @ Operator::Add | op @ Operator::Minus => {
                    result_stack.push_back(OperationUnit::Operator(*op));
                    operation_unit_index += 1;
                }
                _ => return Err(ErrorList::ThisOperatorShouldBeAlreadyComputed),
            },
            Some(n @ OperationUnit::Operand(_)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => return compute_add_minus(&result_stack, verbosity),
        }
    }
}

fn compute_add_minus(operation: &VecDeque<OperationUnit>, verbosity: u8) -> Result<f64, ErrorList> {
    if verbosity > 1 {
        println!("compute_add_minus operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<f64> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(OperationUnit::Operator(op)) => match op {
                Operator::Add => match operation.get(operation_unit_index + 1) {
                    Some(OperationUnit::Operand(next_operand)) => {
                        let last_operand = result_stack.pop_back().unwrap();
                        result_stack.push_back(last_operand + next_operand);
                        operation_unit_index += 2;
                    }
                    _ => return Err(ErrorList::Unknown),
                },
                Operator::Minus => match operation.get(operation_unit_index + 1) {
                    Some(OperationUnit::Operand(next_operand)) => {
                        let last_operand = result_stack.pop_back().unwrap();
                        result_stack.push_back(last_operand - next_operand);
                        operation_unit_index += 2;
                    }
                    _ => return Err(ErrorList::Unknown),
                },
                _ => return Err(ErrorList::ThisOperatorShouldBeAlreadyComputed),
            },
            Some(OperationUnit::Operand(n)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => {
                if result_stack.len() > 1 {
                    return Err(ErrorList::Unknown);
                } else {
                    return match result_stack.pop_back() {
                        Some(n) => Ok(n),
                        None => Err(ErrorList::Unknown),
                    };
                }
            }
        }
    }
}

fn factorial(n: f64, verbosity: u8) -> f64 {
    let mut result = 1.0;
    let mut current = n;
    loop {
        if verbosity > 4 {
            println!(
                "factorial {:?} - loop current={:?} result={:?}",
                n, current, result
            )
        }
        if current < 1.5 {
            return result;
        } else {
            result = result * current;
            current = current - 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! assert_ok {
        ($calculation: expr, $result: literal) => {
            let expected: Result<f64, ErrorList> = Ok($result);
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
