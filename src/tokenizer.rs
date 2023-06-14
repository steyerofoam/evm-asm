use maplit::hashmap;
use std::fmt;

#[derive(Clone)]
pub struct Loc {
	pub line: u64,
	pub col: u64,
	pub filename: String,
}

impl fmt::Display for Loc {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "line {}, column {} in {}", self.line, self.col, self.filename)
	}
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum TokenType {
	Eof,
	Nil,
	Number(String),
	String(String),
	Boolean(bool),
	LeftSquare,
	RightSquare,
	LeftCurly,
	RightCurly,
	Push,
	Dup,
	Swap,
	ILoad,
	Load,
	Drop,
	Query,
	Info,
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

fn get_token_name(typ: &TokenType) -> &str {
	match typ {
		TokenType::Eof           => "end-of-file",
		TokenType::Number(x)     => x,
		TokenType::String(x)     => x,
		TokenType::Boolean(x)    => if *x {"true"} else {"false"},
		TokenType::Nil           => "nil",
		TokenType::LeftSquare    => "[",
		TokenType::RightSquare   => "]",
		TokenType::LeftCurly     => "{",
		TokenType::RightCurly    => "}",
		TokenType::Push          => "push",
		TokenType::Dup           => "dup",
		TokenType::Swap          => "swap",
		TokenType::ILoad         => "iload",
		TokenType::Load          => "load",
		TokenType::Drop          => "drop",
		TokenType::Query         => "query",
		TokenType::Info          => "info",
		TokenType::Each          => "each",
		TokenType::Reduce        => "reduce",
		TokenType::Reverse       => "reverse",
		TokenType::Map           => "map",
		TokenType::Filter        => "filter",
		TokenType::Call          => "call",
		TokenType::ToStr         => "tostr",
		TokenType::ToNum         => "tonum",
		TokenType::Add           => "+",
		TokenType::Sub           => "-",
		TokenType::Mul           => "*",
		TokenType::Div           => "/",
		TokenType::Mod           => "%",
		TokenType::Eq            => "=",
		TokenType::NotEq         => "!=",
		TokenType::Greater       => ">",
		TokenType::GreaterEq     => ">=",
		TokenType::Less          => "<",
		TokenType::LessEq        => "<=",
		TokenType::And           => "and",
		TokenType::Or            => "or",
		TokenType::Not           => "not",
		TokenType::Concat        => "concat",
		TokenType::Match         => "match",
		TokenType::Split         => "split",
		TokenType::Iota          => "iota"
	}
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TokenType::String(x) => write!(f, "\"{}\"", x),
			x => write!(f, "`{}`", get_token_name(x))
		}
	}
}

#[derive(Clone)]
pub struct Token {
	pub typ: TokenType,
	pub loc: Loc,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} at {}", self.typ, self.loc)
	}
}

impl Token {
	pub fn new(typ: TokenType, loc: Loc) -> Token {
		Token {typ, loc}
	}
}

pub fn tokenize(char_str: &str, filename: &str) -> Result<Vec<Token>, String> {
	let mut tokens = vec![];
	let chars: Vec<_> = char_str.chars().collect();

	let mut i = 0;
	let mut line: u64 = 1;
	let mut col: u64 = 1;
	let mut buffer = String::new();

	let token_map = hashmap! {
		'[' => TokenType::LeftSquare,
		']' => TokenType::RightSquare,
		'{' => TokenType::LeftCurly,
		'}' => TokenType::RightCurly,
	};

	let op_map = hashmap! {
		"push".to_owned() => TokenType::Push,
		"dup".to_owned() => TokenType::Dup,
		"swap".to_owned() => TokenType::Swap,
		"iload".to_owned() => TokenType::ILoad,
		"load".to_owned() => TokenType::Load,
		"drop".to_owned() => TokenType::Drop,
		"query".to_owned() => TokenType::Query,
		"info".to_owned() => TokenType::Info,
		"each".to_owned() => TokenType::Each,
		"reduce".to_owned() => TokenType::Reduce,
		"reverse".to_owned() => TokenType::Reverse,
		"map".to_owned() => TokenType::Map,
		"filter".to_owned() => TokenType::Filter,
		"call".to_owned() => TokenType::Call,
		"tostr".to_owned() => TokenType::ToStr,
		"tonum".to_owned() => TokenType::ToNum,
		"+".to_owned() => TokenType::Add,
		"-".to_owned() => TokenType::Sub,
		"*".to_owned() => TokenType::Mul,
		"/".to_owned() => TokenType::Div,
		"%".to_owned() => TokenType::Mod,
		"=".to_owned() => TokenType::Eq,
		"!=".to_owned() => TokenType::NotEq,
		">".to_owned() => TokenType::Greater,
		">=".to_owned() => TokenType::GreaterEq,
		"<".to_owned() => TokenType::Less,
		"<=".to_owned() => TokenType::LessEq,
		"and".to_owned() => TokenType::And,
		"or".to_owned() => TokenType::Or,
		"not".to_owned() => TokenType::Not,
		"concat".to_owned() => TokenType::Concat,
		"match".to_owned() => TokenType::Match,
		"split".to_owned() => TokenType::Split,
		"iota".to_owned() => TokenType::Iota
	};

	macro_rules! here {
		() => {Loc {line, col, filename: filename.to_string()}}
	}

	// shebang check
	if (chars.len() >= 2) && (chars[0] == '#') && (chars[1] == '!') {
		while i < chars.len() { // consume until newline
			let c = chars[i];

			if (c == '\r') || (c == '\n') {
				break;
			}

			i += 1;
		}
	}

	while i < chars.len() {
		let c = chars[i];

		if c == '\r' {
			line += 1;
			col = 0;
			if (i + 1 < chars.len()) && chars[i + 1] == '\n' {i += 1}
		} else if c == '\n' {
			line += 1;
			col = 0;
		} else if c == ';' {
			i += 1;

			while i < chars.len() { // consume until newline
				let nc = chars[i];

				if (nc == '\r') || (nc == '\n') {
					i -= 1;

					break;
				}

				i += 1;
			}
		} else if c.is_whitespace() {
			// do nothing
		} else if token_map.contains_key(&c) {
			tokens.push(Token::new(token_map[&c].clone(), here!()));
		} else if c.is_ascii_digit() { // number takes precendence over identifier because it isolates ascii digits
			buffer += &c.to_string();

			let scol = col;

			let mut found_dot: bool = false;

			while i + 1 < chars.len() {
				i += 1;
				col += 1;

				if chars[i].is_ascii_digit() {
					buffer += &chars[i].to_string();
				} else if (chars[i] == '.') && !found_dot {
					found_dot = true;
					buffer += ".";
				} else {
					i -= 1;
					col -= 1;
					break;
				}
			}

			tokens.push(Token::new(TokenType::Number(buffer.clone()), Loc {line, col: scol, filename: filename.to_string()}));

			buffer.clear();
		} else if c == '.' {
			buffer += &c.to_string();

			let scol = col;

			while i + 1 < chars.len() {
				i += 1;
				col += 1;

				if chars[i].is_ascii_digit() {
					buffer += &chars[i].to_string();
				} else {
					i -= 1;
					col -= 1;
					break;
				}
			}

			tokens.push(Token::new(TokenType::Number(buffer.clone()), Loc {line, col: scol, filename: filename.to_string()}));

			buffer.clear();
		} else if c == '-' {
			if (i + 1 < chars.len()) && ((chars[i + 1] == '.') || chars[i + 1].is_ascii_digit()) {
				buffer += &c.to_string();

				let scol = col;

				let mut found_dot: bool = false;

				while i + 1 < chars.len() {
					i += 1;
					col += 1;

					if chars[i].is_ascii_digit() {
						buffer += &chars[i].to_string();
					} else if (chars[i] == '.') && !found_dot {
						found_dot = true;
						buffer += ".";
					} else {
						i -= 1;
						col -= 1;
						break;
					}
				}

				tokens.push(Token::new(TokenType::Number(buffer.clone()), Loc {line, col: scol, filename: filename.to_string()}));

				buffer.clear();
			} else {
				buffer += &c.to_string();

				let scol = col;

				while (i + 1 < chars.len()) && !chars[i + 1].is_whitespace() && (chars[i + 1] != '"') && !token_map.contains_key(&chars[i + 1]) {
					i += 1;
					col += 1;

					buffer += &chars[i].to_string();
				}

				if op_map.contains_key(&buffer) {
					tokens.push(Token::new(op_map[&buffer].clone(), Loc {line, col: scol, filename: filename.to_string()}))
				}

				buffer.clear();
			}
		} else if c == '"' {
			let sline = line;
			let scol = col;

			while (i + 1 < chars.len()) && (chars[i + 1] != '"') { //"
				i += 1;
				col += 1;

				buffer += &chars[i].to_string();
			}

			i += 1; // skip final quote
			col += 1;

			if i == chars.len() {
				return Err(format!("Unterminated string starting on {}", Loc {line: sline, col: scol, filename: filename.to_string()}));
			}

			tokens.push(Token::new(TokenType::String(buffer.clone()), Loc {line: sline, col: scol, filename: filename.to_string()}));

			buffer.clear();
		} else {
			buffer += &c.to_string();

			let scol = col;

			while (i + 1 < chars.len()) && !chars[i + 1].is_whitespace() && (chars[i + 1] != '"') && !token_map.contains_key(&chars[i + 1]) {
				i += 1;
				col += 1;

				buffer += &chars[i].to_string();
			}

			if op_map.contains_key(&buffer) {
				tokens.push(Token::new(op_map[&buffer].clone(), Loc {line, col: scol, filename: filename.to_string()}))
			}

			buffer.clear();
		}

		i += 1;
	}

	tokens.push(Token::new(TokenType::Eof, here!()));

	Ok(tokens)
}
