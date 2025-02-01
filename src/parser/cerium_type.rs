pub enum CeriumType {
    I16,
    U16,
    F16,
    Any,
    Bool,
    Struct(String, Vec<CeriumType>),
    Function(Vec<CeriumType>, Option<Box<CeriumType>>), // no interest in working with a unit type
    Pointer(Box<CeriumType>),
}