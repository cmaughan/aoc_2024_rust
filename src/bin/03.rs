advent_of_code::solution!(3);

#[derive(Debug)]
enum State {
    Idle,          // No active processing
    ProcessingMul, // In the process of parsing mul(xxx, yyy)
    ProcessingDo,  // In the process of parsing do()
    ProcessingDont, // In the process of parsing don't()
}

#[derive(Debug)]
enum Token {
    Mul(i64, i64),
    Do,
    Dont,
}

struct Tokenizer<'a> {
    state: State,
    ignore: bool,
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Tokenizer {
            state: State::Idle,
            ignore: false,
            input,
            position: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current_char = self.input[self.position..].chars().next().unwrap();
            if current_char.is_whitespace() {
                // Skip whitespace
                self.position += current_char.len_utf8();
                continue;
            }

            match self.state {
                State::Idle => {
                    if self.input[self.position..].starts_with("mul(") {
                        // Handle mul(xxx,yyy) pattern
                        self.state = State::ProcessingMul;
                    } else if self.input[self.position..].starts_with("do()") {
                        // Handle do() pattern
                        self.ignore = false;
                        tokens.push(Token::Do);
                        self.position += 4; // move past "do()"
                    } else if self.input[self.position..].starts_with("don't()") {
                        // Handle don't() pattern
                        self.ignore = true;
                        tokens.push(Token::Dont);
                        self.position += 7; // move past "don't()"
                    } else {
                        self.position += 1; // Skip invalid or unrecognized characters
                    }
                }

                State::ProcessingMul => {
                    // Look for closing parenthesis in the mul(xxx,yyy) pattern
                    if self.input[self.position..].starts_with("mul(") {
                        let start_pos = self.position + 4; // skip "mul("
                        let mut end_pos = self.input[start_pos..].find(')').unwrap() + start_pos;
                        let mul_content = &self.input[start_pos..end_pos];

                        let parts: Vec<&str> = mul_content.split(',').collect();
                        if parts.len() == 2 {
                            let x_str = parts[0].trim();
                            let y_str = parts[1].trim();

                            // Validate if both x and y have 1 to 3 digits
                            if x_str.len() <= 3 && y_str.len() <= 3 && x_str.len() >= 1 && y_str.len() >= 1 {
                                // Try to parse both as numbers
                                if let (Ok(x_val), Ok(y_val)) = (x_str.parse::<i64>(), y_str.parse::<i64>()) {
                                    tokens.push(Token::Mul(x_val, y_val));
                                }
                            }
                            else {
                               end_pos = self.position + 3;
                            }
                        }
                        else
                        {
                            end_pos = self.position + 3;
                        }
                        self.position = end_pos + 1; // Skip past the closing parenthesis
                        self.state = State::Idle; // Reset state to Idle
                    }
                }

                State::ProcessingDo | State::ProcessingDont => {
                    // No additional processing needed for do() or don't() once we've already handled them
                    self.state = State::Idle;
                }
            }
        }

        tokens
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tokenizer = Tokenizer::new(input);

    let tokens = tokenizer.tokenize();
    let mut ret = 0;

    for token in tokens {
        match token {
            Token::Mul(x, y) => ret = ret + (x * y),
            Token::Do => (),
            Token::Dont => ()
        }
    }

    Some(ret as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tokenizer = Tokenizer::new(input);

    let tokens = tokenizer.tokenize();
    let mut ret = 0;

    let mut ignore = false;
    for token in tokens {
        match token {
            Token::Mul(x, y) => if !ignore {ret = ret + (x * y)},
            Token::Do => ignore = false,
            Token::Dont => ignore = true
        }
    }

    Some(ret as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
