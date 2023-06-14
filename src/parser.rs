use std::cell::Cell;
use std::fmt;

use crate::tokenizer::*;

pub struct State {
	ctok: Cell<usize>,
	tokens: Vec<Token>
}

#[derive(Clone, PartialEq)]
pub enum Value {
	Number(f64),
	String(String),
	Function(Vec<Command>),
	Array(Vec<Value>)
}

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum Command {
	Push(Value),
	Dup,
	Swap,
	ILoad(u64, Value),
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
			Value::Number(val)     => write!(f, "{}", val),
			Value::String(val)     => write!(f, "\"{}\"", val),
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

fn expect(state: &State, typ: &TokenType) -> Result<(), String> {
	if !accept(state, typ) {
		return Err(format!("Unexpected token: expected {}, got {} on {}", typ, last(state).typ, last(state).loc));
	}
	Ok(())
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
	} else {
		let t = next(state);

		Err(format!("Unexpected token {} on {}", t.typ, t.loc))
	}
}

fn parse_command(state: &State) -> Result<Command, String> {
	let t = next(state);

	match t.typ {
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
