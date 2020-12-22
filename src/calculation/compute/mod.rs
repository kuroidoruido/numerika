use std::collections::VecDeque;

use crate::calculation::models;

pub fn compute(
    operation: &VecDeque<models::OperationUnit>,
    verbosity: u8,
) -> Result<f64, models::ErrorList> {
    if verbosity > 1 {
        println!("compute operation: {:?}", operation);
    }
    return compute_unary(&operation, verbosity);
}

fn compute_unary(
    operation: &VecDeque<models::OperationUnit>,
    verbosity: u8,
) -> Result<f64, models::ErrorList> {
    if verbosity > 1 {
        println!("compute_unary operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<models::OperationUnit> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(models::OperationUnit::Operator(op)) => match op {
                models::Operator::Factorial => match result_stack.pop_back() {
                    Some(models::OperationUnit::Operator(_)) => {
                        return Err(models::ErrorList::InvalidOperatorPosition)
                    }
                    Some(models::OperationUnit::Operand(n)) => {
                        result_stack
                            .push_back(models::OperationUnit::Operand(factorial(n, verbosity)));
                        operation_unit_index += 1;
                    }
                    None => return Err(models::ErrorList::InvalidOperatorPosition),
                },
                models::Operator::Add => match result_stack.back() {
                    Some(models::OperationUnit::Operand(_)) => {
                        result_stack
                            .push_back(models::OperationUnit::Operator(models::Operator::Add));
                        operation_unit_index += 1;
                    }
                    _ => {
                        operation_unit_index += 1;
                        // nothing to do we can ignore it because the case is one of these: "+2", "2++2", "2*+2", ...
                    }
                },
                models::Operator::Minus => match result_stack.back() {
                    Some(models::OperationUnit::Operand(_)) => {
                        result_stack
                            .push_back(models::OperationUnit::Operator(models::Operator::Minus));
                        operation_unit_index += 1;
                    }
                    _ => match operation.get(operation_unit_index + 1) {
                        Some(models::OperationUnit::Operand(n)) => {
                            result_stack.push_back(models::OperationUnit::Operand(-n));
                            operation_unit_index += 2;
                        }
                        _ => return Err(models::ErrorList::InvalidOperatorPosition),
                    },
                },
                op => {
                    result_stack.push_back(models::OperationUnit::Operator(*op));
                    operation_unit_index += 1;
                }
            },
            Some(n @ models::OperationUnit::Operand(_)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => return compute_multiply_divide(&result_stack, verbosity),
        }
    }
}

fn compute_multiply_divide(
    operation: &VecDeque<models::OperationUnit>,
    verbosity: u8,
) -> Result<f64, models::ErrorList> {
    if verbosity > 1 {
        println!("compute_multiply_divide operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<models::OperationUnit> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(models::OperationUnit::Operator(op)) => match op {
                models::Operator::Multiply => match result_stack.pop_back() {
                    Some(models::OperationUnit::Operator(_)) => {
                        return Err(models::ErrorList::InvalidOperatorPosition)
                    }
                    Some(models::OperationUnit::Operand(last_operand)) => {
                        match operation.get(operation_unit_index + 1) {
                            Some(models::OperationUnit::Operator(_)) => {
                                return Err(models::ErrorList::InvalidOperatorPosition)
                            }
                            Some(models::OperationUnit::Operand(next_operand)) => {
                                result_stack.push_back(models::OperationUnit::Operand(
                                    last_operand * next_operand,
                                ));
                                operation_unit_index += 2;
                            }
                            None => return Err(models::ErrorList::InvalidOperatorPosition),
                        }
                    }
                    None => return Err(models::ErrorList::InvalidOperatorPosition),
                },
                models::Operator::Divide => match result_stack.pop_back() {
                    Some(models::OperationUnit::Operator(_)) => {
                        return Err(models::ErrorList::InvalidOperatorPosition)
                    }
                    Some(models::OperationUnit::Operand(last_operand)) => {
                        match operation.get(operation_unit_index + 1) {
                            Some(models::OperationUnit::Operator(_)) => {
                                return Err(models::ErrorList::InvalidOperatorPosition)
                            }
                            Some(models::OperationUnit::Operand(next_operand)) => {
                                result_stack.push_back(models::OperationUnit::Operand(
                                    last_operand / next_operand,
                                ));
                                operation_unit_index += 2;
                            }
                            None => return Err(models::ErrorList::InvalidOperatorPosition),
                        }
                    }
                    None => return Err(models::ErrorList::InvalidOperatorPosition),
                },
                op @ models::Operator::Add | op @ models::Operator::Minus => {
                    result_stack.push_back(models::OperationUnit::Operator(*op));
                    operation_unit_index += 1;
                }
                _ => return Err(models::ErrorList::ThisOperatorShouldBeAlreadyComputed),
            },
            Some(n @ models::OperationUnit::Operand(_)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => return compute_add_minus(&result_stack, verbosity),
        }
    }
}

fn compute_add_minus(
    operation: &VecDeque<models::OperationUnit>,
    verbosity: u8,
) -> Result<f64, models::ErrorList> {
    if verbosity > 1 {
        println!("compute_add_minus operation: {:?}", operation);
    }
    let mut result_stack: VecDeque<f64> = VecDeque::new();
    let mut operation_unit_index = 0;
    loop {
        match operation.get(operation_unit_index) {
            Some(models::OperationUnit::Operator(op)) => match op {
                models::Operator::Add => match operation.get(operation_unit_index + 1) {
                    Some(models::OperationUnit::Operand(next_operand)) => {
                        let last_operand = result_stack.pop_back().unwrap();
                        result_stack.push_back(last_operand + next_operand);
                        operation_unit_index += 2;
                    }
                    _ => return Err(models::ErrorList::Unknown),
                },
                models::Operator::Minus => match operation.get(operation_unit_index + 1) {
                    Some(models::OperationUnit::Operand(next_operand)) => {
                        let last_operand = result_stack.pop_back().unwrap();
                        result_stack.push_back(last_operand - next_operand);
                        operation_unit_index += 2;
                    }
                    _ => return Err(models::ErrorList::Unknown),
                },
                _ => return Err(models::ErrorList::ThisOperatorShouldBeAlreadyComputed),
            },
            Some(models::OperationUnit::Operand(n)) => {
                result_stack.push_back(*n);
                operation_unit_index += 1;
            }
            None => {
                if result_stack.len() > 1 {
                    return Err(models::ErrorList::Unknown);
                } else {
                    return match result_stack.pop_back() {
                        Some(n) => Ok(n),
                        None => Err(models::ErrorList::Unknown),
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
