trait Library {
	fn library(&self) -> Option<&'static str>;
}

trait CodeGen {
	fn gen(&self) -> Option<&'static str>;
}

#[derive(PartialEq, Eq)]
pub enum Sign {
	S,
	U
}

pub enum TypeSize {
	BYTE,
	WORD,
	DWORD,
	QWORD
}

pub enum Type {
	Int(TypeSize, Sign),
	Float,
	String,
	Str,
	Char,
}

impl Library for Type {
	fn library(&self) -> Option<&'static str> {
		match self {
			Type::Int(..) => Some("stdint.h"),
			_ => None
		}
	}
}

impl CodeGen for Type {
	fn gen(&self) -> Option<&'static str> {
		match self {
			Type::Int(size, sign) => {
				match size {
					TypeSize::BYTE if sign == &Sign::U => Some("uint8_t"),
					TypeSize::WORD if sign == &Sign::U => Some("uint16_t"),
					TypeSize::DWORD if sign == &Sign::U => Some("uint32_t"),
					TypeSize::QWORD if sign == &Sign::U => Some("uint64_t"),

					TypeSize::BYTE if sign == &Sign::S => Some("int8_t"),
					TypeSize::WORD if sign == &Sign::S => Some("int16_t"),
					TypeSize::DWORD if sign == &Sign::S => Some("int32_t"),
					TypeSize::QWORD if sign == &Sign::S => Some("int64_t"),
					_ => None
				}
			}
			_ => todo!()
		}
	}
}

pub enum Ast {
	Sequence(Vec<Ast>)
}
