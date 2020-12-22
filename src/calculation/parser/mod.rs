use std::collections::VecDeque;

use crate::calculation::models;

pub fn parse_operation(
    str_calculation: &String,
    verbosity: u8,
) -> Result<VecDeque<models::OperationUnit>, models::ErrorList> {
    let mut calc_iter = str_calculation.chars();
    let mut operation: VecDeque<models::OperationUnit> = VecDeque::new();
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
                            operation.push_back(models::OperationUnit::Operand(n));
                            current_number.clear();
                        }
                        Err(err) => return Err(err),
                    }
                }
                operation.push_back(models::OperationUnit::Operator(
                    models::OperationUnit::from_char(op).unwrap(),
                ));
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
                            operation.push_back(models::OperationUnit::Operand(n));
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

fn parse_number(current_number: &String) -> Result<f64, models::ErrorList> {
    return match current_number.parse::<f64>() {
        Err(_) => Err(models::ErrorList::CannotParseNumber(current_number.clone())),
        Ok(n) => Ok(n),
    };
}
