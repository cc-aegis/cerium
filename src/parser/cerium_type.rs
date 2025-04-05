use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CeriumType {
    I16,
    U16,
    F16,
    Any,
    Bool,
    Struct(String),
    Function(Vec<CeriumType>, Option<Box<CeriumType>>), // no interest in working with a unit type
    Pointer(Box<CeriumType>),
}

impl Display for CeriumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CeriumType::I16 => write!(f, "i16"),
            CeriumType::U16 => write!(f, "u16"),
            CeriumType::F16 => write!(f, "f16"),
            CeriumType::Any => write!(f, "any"),
            CeriumType::Bool => write!(f, "bool"),
            CeriumType::Struct(name) => write!(f, "{name}"),
            CeriumType::Function(params, Some(return_type)) => {
                write!(f, "fn({}) -> {return_type}", join(params, ", "))
            },
            CeriumType::Function(params, None) => {
                write!(f, "fn({})", join(params, ", "))
            },
            CeriumType::Pointer(ty) => write!(f, "&{ty}"),
        }
    }
}

fn join<T: Display>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(T::to_string)
        .collect::<Vec<String>>()
        .join(sep)
}

pub fn format_type(c_type: &Option<CeriumType>) -> String {
    match c_type {
        Some(c_type) => c_type.to_string(),
        None => String::from("unit"),
    }
}