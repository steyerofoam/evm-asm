use bytes::{BytesMut, BufMut};
use crate::parser::*;

pub fn generate_value(value: Value) -> BytesMut {
	let mut buf = BytesMut::new();

	buf.put_u8(unsafe {*<*const _>::from(&value).cast::<u8>()}); // safe because of repr(u8) on enum

	match value {
		Value::Nil => {},
		Value::Number(val) => buf.put_slice(&val.to_le_bytes()),
		Value::String(val) => {
			buf.put_u64_le(val.len() as u64);
			buf.put_slice((&val).as_bytes())
		},
		Value::Boolean(val) => buf.put_u8(val as u8),
		Value::Function(commands) => {
			buf.put_u64_le(commands.len() as u64);
			buf.extend_from_slice(&generate(commands));
		},
		Value::Array(values) => {
			buf.put_u64_le(values.len() as u64);

			for value in values {
				buf.extend_from_slice(&generate_value(value))
			}
		}
	}

	buf
}

pub fn generate(commands: Vec<Command>) -> BytesMut {
	let mut buf = BytesMut::new();

	for command in commands {
		buf.put_u8(unsafe {*<*const _>::from(&command).cast::<u8>()}); // safe because of repr(u8) on enum

		match command {
			Command::Push(value) => buf.extend_from_slice(&generate_value(value)),
			Command::ILoad(reg, value) => {
				buf.put_u8(reg);
				buf.extend_from_slice(&generate_value(value));
			},
			_ => {}
		}
	}

	buf
}