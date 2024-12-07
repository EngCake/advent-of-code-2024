use std::{io::stdin, iter::Peekable, str::Chars};

fn main() {
    let mut state_machine = StateMachine::new();
    while let Some(line) = read_line() {
        state_machine.process(&line);
    }
    println!("{}", state_machine.result);
}

#[derive(Debug, Clone)]
struct StateMachine {
    state: State,
    enabled: bool,
    first_number: String,
    second_number: String,
    result: i32,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Root,
    Multiply(MuliplyState),
    Do(DoState),
    Dont(DontState),
}

#[derive(Debug, Clone, Copy)]
enum MuliplyState {
    Keyword(i32),
    LeftParenthesis,
    FirstNumber,
    Comma,
    SecondNumber,
    RightParenthesis,
}

#[derive(Debug, Clone, Copy)]
enum DoState {
    Keyword(i32),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, Clone, Copy)]
enum DontState {
    Keyword(i32),
    LeftParenthesis,
    RightParenthesis,
}

impl StateMachine {
    fn new() -> StateMachine {
        return StateMachine {
            state: State::Root,
            enabled: true,
            first_number: String::new(),
            second_number: String::new(),
            result: 0,
        };
    }

    fn process(&mut self, line: &String) {
        let mut iter = line.chars().peekable();
        self.first_number.clear();
        self.second_number.clear();
        while let Some(_) = self.next(&mut iter) {}
    }

    fn next<'a>(&mut self, iter: &mut Peekable<Chars<'a>>) -> Option<()> {
        if let Some(&letter) = iter.peek() {
            return Some(match self.state {
                State::Root => self.process_root(letter, iter),
                State::Multiply(state) => self.process_multiply(state, letter, iter),
                State::Do(state) => self.process_do(state, letter, iter),
                State::Dont(state) => self.process_dont(state, letter, iter),
            });
        } else {
            return None;
        }
    }

    fn process_root<'a>(&mut self, letter: char, iter: &mut Peekable<Chars<'a>>) {
        iter.next();
        match letter {
            'm' => self.state = State::Multiply(MuliplyState::Keyword(1)),
            'd' => self.state = State::Do(DoState::Keyword(1)),
            _ => (),
        };
    }

    fn process_multiply<'a>(
        &mut self,
        state: MuliplyState,
        letter: char,
        iter: &mut Peekable<Chars<'a>>,
    ) {
        match state {
            MuliplyState::Keyword(index) => match index {
                1 => match letter {
                    'u' => {
                        self.state = State::Multiply(MuliplyState::Keyword(2));
                        iter.next();
                    }
                    _ => self.set_root_state(),
                },
                2 => match letter {
                    'l' => {
                        self.state = State::Multiply(MuliplyState::LeftParenthesis);
                        iter.next();
                    }
                    _ => self.set_root_state(),
                },
                _ => panic!("reached unreachable state"),
            },
            MuliplyState::LeftParenthesis => match letter {
                '(' => {
                    self.state = State::Multiply(MuliplyState::FirstNumber);
                    iter.next();
                }
                _ => self.set_root_state(),
            },
            MuliplyState::FirstNumber => match letter {
                _ if letter.is_ascii_digit() && self.first_number.len() <= 3 => {
                    self.first_number.push(letter);
                    iter.next();
                }
                ',' => {
                    self.state = State::Multiply(MuliplyState::Comma);
                }
                _ => self.set_root_state(),
            },
            MuliplyState::Comma => match letter {
                ',' => {
                    self.state = State::Multiply(MuliplyState::SecondNumber);
                    iter.next();
                }
                _ => self.set_root_state(),
            },
            MuliplyState::SecondNumber => match letter {
                _ if letter.is_ascii_digit() && self.second_number.len() <= 3 => {
                    self.second_number.push(letter);
                    iter.next();
                }
                ')' => {
                    self.state = State::Multiply(MuliplyState::RightParenthesis);
                }
                _ => self.set_root_state(),
            },
            MuliplyState::RightParenthesis => match letter {
                ')' => {
                    iter.next();
                    if self.enabled {
                        let first_number = self.first_number.parse::<i32>().unwrap();
                        let second_number = self.second_number.parse::<i32>().unwrap();
                        self.result += first_number * second_number;
                    }
                    self.set_root_state()
                }
                _ => self.set_root_state(),
            },
        }
    }

    fn process_do<'a>(&mut self, state: DoState, letter: char, iter: &mut Peekable<Chars<'a>>) {
        match state {
            DoState::Keyword(index) => match index {
                1 if letter == 'o' => {
                    self.state = State::Do(DoState::LeftParenthesis);
                    iter.next();
                }
                _ => self.set_root_state(),
            },
            DoState::LeftParenthesis => match letter {
                '(' => {
                    self.state = State::Do(DoState::RightParenthesis);
                    iter.next();
                }
                'n' => {
                    self.state = State::Dont(DontState::Keyword(3));
                    iter.next();
                }
                _ => self.set_root_state(),
            },
            DoState::RightParenthesis => match letter {
                ')' => {
                    self.enabled = true;
                    iter.next();
                    self.set_root_state()
                }
                _ => self.set_root_state(),
            },
        }
    }

    fn process_dont<'a>(&mut self, state: DontState, letter: char, iter: &mut Peekable<Chars<'a>>) {
        match state {
            DontState::Keyword(index) => match index {
                3 if letter == '\'' => {
                    self.state = State::Dont(DontState::Keyword(4));
                    iter.next();
                }
                4 if letter == 't' => {
                    self.state = State::Dont(DontState::LeftParenthesis);
                    iter.next();
                }
                _ => panic!("reached unreachable state"),
            },
            DontState::LeftParenthesis => match letter {
                '(' => {
                    self.state = State::Dont(DontState::RightParenthesis);
                    iter.next();
                }
                _ => self.set_root_state(),
            },
            DontState::RightParenthesis => match letter {
                ')' => {
                    self.enabled = false;
                    iter.next();
                    self.set_root_state();
                }
                _ => self.set_root_state(),
            },
        }
    }

    fn set_root_state(&mut self) {
        self.state = State::Root;
        self.first_number.clear();
        self.second_number.clear();
    }
}

fn read_line() -> Option<String> {
    let mut buffer = String::new();
    if stdin().read_line(&mut buffer).ok()? == 0 {
        return None;
    }
    if buffer == "\n" {
        return None;
    }
    return Some(buffer);
}
