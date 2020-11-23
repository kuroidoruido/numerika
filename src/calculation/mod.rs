use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
}

fn operator_from_char(char_op: char) -> Option<Operator> {
    match char_op {
        '+' => Some(Operator::Add),
        '-' => Some(Operator::Minus),
        '*' => Some(Operator::Multiply),
        '/' => Some(Operator::Divide),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorList {
    Unknown,
}

pub fn parse_and_compute(str_calculation: String, verbosity: u8) -> Result<f64, ErrorList> {
    let mut calc_iter = str_calculation.chars();
    let mut number_list: VecDeque<f64> = VecDeque::new();
    let mut operator_list: VecDeque<Operator> = VecDeque::new();
    let mut current_number: String = String::from("");
    loop {
        match calc_iter.next() {
            Some(n @ '1') | Some(n @ '2') | Some(n @ '3') | Some(n @ '4') | Some(n @ '5')
            | Some(n @ '6') | Some(n @ '7') | Some(n @ '8') | Some(n @ '9') | Some(n @ '0')
            | Some(n @ '.') => current_number.push(n),
            Some(op @ '+') | Some(op @ '-') | Some(op @ '*') | Some(op @ '/') => handle_operator(
                &mut number_list,
                &mut current_number,
                &mut operator_list,
                operator_from_char(op).unwrap(),
            ),
            Some(other @ _) => {
                if verbosity > 1 {
                    println!("Ignore {:?}", other);
                }
            }
            None => {
                number_list.push_back(parse_number(&current_number));
                current_number.clear();
                return compute(&number_list, &operator_list, verbosity);
            }
        }
    }
}

fn handle_operator(
    number_list: &mut VecDeque<f64>,
    current_number: &mut String,
    operator_list: &mut VecDeque<Operator>,
    operator: Operator,
) {
    number_list.push_back(parse_number(&current_number));
    current_number.clear();
    operator_list.push_back(operator);
}

fn parse_number(current_number: &String) -> f64 {
    // TODO fix this because can be safer
    return current_number.parse::<f64>().unwrap();
}

fn compute(
    number_list: &VecDeque<f64>,
    operator_list: &VecDeque<Operator>,
    verbosity: u8,
) -> Result<f64, ErrorList> {
    if verbosity > 1 {
        println!("All parsed numbers: {:?}", number_list);
        println!("All parsed operators: {:?}", operator_list);
    }
    let mut result_stack: VecDeque<f64> = VecDeque::new();
    result_stack.push_back(*number_list.get(0).unwrap());
    let mut waiting_operator_stack: VecDeque<Operator> = VecDeque::new();
    waiting_operator_stack.push_back(*operator_list.get(0).unwrap());
    let mut number_list_index = 1;
    let mut operator_list_index = 1;
    loop {
        let next_operator = operator_list.get(operator_list_index);
        operator_list_index += 1;
        let current_number = number_list.get(number_list_index).unwrap();
        number_list_index += 1;
        match next_operator {
            Some(op @ Operator::Add) | Some(op @ Operator::Minus) => {
                if verbosity > 3 {
                    println!("Handle {:?}", op);
                }
                result_stack.push_back(*current_number);
                resolve_stacks(&mut result_stack, &mut waiting_operator_stack, verbosity);
                waiting_operator_stack.push_back(*op);
            }
            Some(op @ Operator::Multiply) | Some(op @ Operator::Divide) => {
                if verbosity > 3 {
                    println!("Handle {:?}", op);
                }
                result_stack.push_back(*current_number);
                waiting_operator_stack.push_back(*op);
            }
            None => {
                result_stack.push_back(*current_number);
                if verbosity > 3 {
                    println!("End of calculation {:?}", result_stack);
                }
                resolve_stacks(&mut result_stack, &mut waiting_operator_stack, verbosity);
                match result_stack.pop_back() {
                    Some(n) => return Ok(n),
                    None => return Err(ErrorList::Unknown),
                }
            }
        }
    }
}

fn resolve_stacks(
    result_stack: &mut VecDeque<f64>,
    waiting_operator_stack: &mut VecDeque<Operator>,
    verbosity: u8,
) {
    if verbosity > 2 {
        print!("Resolve: {:?} {:?}", result_stack, waiting_operator_stack);
    }
    for operator in waiting_operator_stack.iter().rev() {
        let top_number_stack1 = result_stack.pop_back().unwrap();
        let top_number_stack2 = result_stack.pop_back().unwrap();
        match operator {
            Operator::Add => result_stack.push_back(top_number_stack2 + top_number_stack1),
            Operator::Minus => result_stack.push_back(top_number_stack2 - top_number_stack1),
            Operator::Multiply => result_stack.push_back(top_number_stack2 * top_number_stack1),
            Operator::Divide => result_stack.push_back(top_number_stack2 / top_number_stack1),
        }
    }
    waiting_operator_stack.clear();
    if verbosity > 2 {
        println!(" => {:?}", result_stack);
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
    fn it_should_parse_one_operator_add() {
        assert_ok!("3+2", 5.0);
    }

    #[test]
    fn it_should_parse_one_operator_minus() {
        assert_ok!("3-2", 1.0);
    }

    #[test]
    fn it_should_parse_one_operator_multiply() {
        assert_ok!("3*2", 6.0);
    }

    #[test]
    fn it_should_parse_one_operator_divide() {
        assert_ok!("3/2", 1.5);
    }

    #[test]
    fn it_should_parse_multiple_same_operator() {
        assert_ok!("4+3+2+1", 10.0);
    }

    #[test]
    fn it_should_parse_mix_add_minus_operator() {
        assert_ok!("4-3+2-1", 2.0);
    }

    #[test]
    fn it_should_parse_add_multiply_operator_priority() {
        assert_ok!("4+3*2", 10.0);
        assert_ok!("4+3*2+1", 11.0);
    }

    #[test]
    fn it_should_parse_add_divide_operator_priority() {
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
}
