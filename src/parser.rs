use std::cell::Cell;
use std::fmt;

use crate::tokenizer::*;

pub struct State {
	ctok: Cell<usize>,
	tokens: Vec<Token>
}

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum Value {
	Nil,
	Number(f64),
	String(String),
	Boolean(bool),
	Function(Vec<Command>),
	Array(Vec<Value>)
}

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum Command {
	Push(Value),
	Dup,
	Swap,
	ILoad(u8, Value),
	Load,
	Drop,
	Query,
	Info,
	If,
	Each,
	Reduce,
	Reverse,
	Map,
	Filter,
	Call,
	ToStr,
	ToNum,
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Eq,
	NotEq,
	Greater,
	GreaterEq,
	Less,
	LessEq,
	And,
	Or,
	Not,
	Concat,
	Match,
	Split,
	Iota
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Nil             => write!(f, "nil"),
			Value::Number(val)     => write!(f, "{}", val),
			Value::String(val)     => write!(f, "\"{}\"", val),
			Value::Boolean(val)    => write!(f, "{}", val),
			Value::Function(cmds)  => {
				let mut string = "{".to_owned();

				for cmd in cmds {
					if string.len() != 1 {
						string.push(' ');
					}
					string.push_str(&format!("{}", cmd));
				}

				string.push('}');
				write!(f, "{}", string)
			},
			Value::Array(vals) => {
				let mut string = "[".to_owned();

				for val in vals {
					if string.len() != 1 {
						string.push(' ');
					}
					string.push_str(&format!("{}", val));
				}

				string.push(']');
				write!(f, "{}", string)
			}
		}
	}
}

fn get_command_name(cmd: &Command) -> &str {
	match cmd {
		Command::Push(_)    => "push",
		Command::Dup        => "dup",
		Command::Swap       => "swap",
		Command::ILoad(_,_) => "iload",
		Command::Load       => "load",
		Command::Drop       => "drop",
		Command::Query      => "query",
		Command::Info       => "info",
		Command::If         => "if",
		Command::Each       => "each",
		Command::Reduce     => "reduce",
		Command::Reverse    => "reverse",
		Command::Map        => "map",
		Command::Filter     => "filter",
		Command::Call       => "call",
		Command::ToStr      => "tostr",
		Command::ToNum      => "tonum",
		Command::Add        => "+",
		Command::Sub        => "-",
		Command::Mul        => "*",
		Command::Div        => "/",
		Command::Mod        => "%",
		Command::Eq         => "=",
		Command::NotEq      => "!=",
		Command::Greater    => ">",
		Command::GreaterEq  => ">=",
		Command::Less       => "<",
		Command::LessEq     => "<=",
		Command::And        => "and",
		Command::Or         => "or",
		Command::Not        => "not",
		Command::Concat     => "concat",
		Command::Match      => "match",
		Command::Split      => "split",
		Command::Iota       => "iota"
	}
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Command::Push(val) => write!(f, "push {val}"),
			Command::ILoad(reg, val) => write!(f, "iload {reg} {val}"),
			x => write!(f, "{}", get_command_name(x))
		}
	}
}

fn next(state: &State) -> Token {
	state.ctok.set(state.ctok.get() + 1);
	state.tokens[state.ctok.get() - 1].clone()
}

fn last(state: &State) -> Token {
	state.tokens[state.ctok.get() - 1].clone()
}

fn rewind(state: &State, amount: usize) {
	state.ctok.set(state.ctok.get() - amount);
}

fn accept(state: &State, typ: &TokenType) -> bool {
	if &next(state).typ == typ {
		return true;
	}
	rewind(state, 1);
	false
}

fn accept_str(state: &State) -> bool {
	match next(state).typ {
		TokenType::String(_) => {
			true
		},
		_ => {
			rewind(state, 1);
			false
		}
	}
}

fn accept_num(state: &State) -> bool {
	match next(state).typ {
		TokenType::Number(_) => {
			true
		},
		_ => {
			rewind(state, 1);
			false
		}
	}
}

fn accept_bool(state: &State) -> bool {
	match next(state).typ {
		TokenType::Boolean(_) => {
			true
		},
		_ => {
			rewind(state, 1);
			false
		}
	}
}

fn expect_num(state: &State) -> Result<f64, String> {
	match next(state).typ {
		TokenType::Number(val) => {
			match val.parse::<f64>() {
				Ok(parsed) => Ok(parsed),
				Err(error) => Err(format!("Failed to parse number: {}", error))
			}
		},
		_ => {
			rewind(state, 1);
			Err(format!("Unexpected token: expected number, got {} on {}", last(state).typ, last(state).loc))
		}
	}
}

fn parse_value(state: &State) -> Result<Value, String> {
	if accept_num(state) {
		let TokenType::Number(val) = last(state).typ else {unreachable!()};
		match val.parse::<f64>() {
			Ok(parsed) => Ok(Value::Number(parsed)),
			Err(error) => Err(format!("Failed to parse number: {}", error))
		}
	} else if accept_str(state) {
		let TokenType::String(val) = last(state).typ else {unreachable!()};
		Ok(Value::String(val))
	} else if accept_bool(state) {
		let TokenType::Boolean(val) = last(state).typ else {unreachable!()};
		Ok(Value::Boolean(val))
	} else if accept(state, &TokenType::LeftSquare) {
		let mut values = vec![];

		while !accept(state, &TokenType::RightSquare) {
			values.push(parse_value(state)?);
		}

		Ok(Value::Array(values))
	} else if accept(state, &TokenType::LeftCurly) {
		let mut commands = vec![];

		while !accept(state, &TokenType::RightCurly) {
			commands.push(parse_command(state)?);
		}

		Ok(Value::Function(commands))
	} else if accept(state, &TokenType::Nil) {
		Ok(Value::Nil)
	} else {
		let t = next(state);

		Err(format!("Unexpected token {} on {}", t.typ, t.loc))
	}
}

fn parse_command(state: &State) -> Result<Command, String> {
	let t = next(state);

	match t.typ {
		TokenType::Push => {
			let value = parse_value(state)?;

			Ok(Command::Push(value))
		},
		TokenType::ILoad => {
			let reg = expect_num(state)?;

			if reg != reg.trunc() {
				return Err(format!("Register is not an integer: {} on {}", reg, last(state).loc))
			}

			let reg = reg as u64;

			if !(0..16).contains(&reg) {
				return Err(format!("Register must be between 0-15: {} on {}", reg, last(state).loc))
			}

			let value = parse_value(state)?;

			Ok(Command::ILoad(reg as u8, value))
		},
		TokenType::Dup => {Ok(Command::Dup)},
		TokenType::Swap => {Ok(Command::Swap)},
		TokenType::Load => {Ok(Command::Load)},
		TokenType::Drop => {Ok(Command::Drop)},
		TokenType::Query => {Ok(Command::Query)},
		TokenType::Info => {Ok(Command::Info)},
		TokenType::If => {Ok(Command::If)},
		TokenType::Each => {Ok(Command::Each)},
		TokenType::Reduce => {Ok(Command::Reduce)},
		TokenType::Reverse => {Ok(Command::Reverse)},
		TokenType::Map => {Ok(Command::Map)},
		TokenType::Filter => {Ok(Command::Filter)},
		TokenType::Call => {Ok(Command::Call)},
		TokenType::ToStr => {Ok(Command::ToStr)},
		TokenType::ToNum => {Ok(Command::ToNum)},
		TokenType::Add => {Ok(Command::Add)},
		TokenType::Sub => {Ok(Command::Sub)},
		TokenType::Mul => {Ok(Command::Mul)},
		TokenType::Div => {Ok(Command::Div)},
		TokenType::Mod => {Ok(Command::Mod)},
		TokenType::Eq => {Ok(Command::Eq)},
		TokenType::NotEq => {Ok(Command::NotEq)},
		TokenType::Greater => {Ok(Command::Greater)},
		TokenType::GreaterEq => {Ok(Command::GreaterEq)},
		TokenType::Less => {Ok(Command::Less)},
		TokenType::LessEq => {Ok(Command::LessEq)},
		TokenType::And => {Ok(Command::And)},
		TokenType::Or => {Ok(Command::Or)},
		TokenType::Not => {Ok(Command::Not)},
		TokenType::Concat => {Ok(Command::Concat)},
		TokenType::Match => {Ok(Command::Match)},
		TokenType::Split => {Ok(Command::Split)},
		TokenType::Iota => {Ok(Command::Iota)},
		_ => {
			Err(format!("Unexpected token {} on {}", t.typ, t.loc))
		}
	}
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Command>, String> {
	let mut commands = vec![];
	let state = State {
		ctok: Cell::new(0),
		tokens
	};

	while !accept(&state, &TokenType::Eof) {
		commands.push(parse_command(&state)?);
	}

	Ok(commands)
}
