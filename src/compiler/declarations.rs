use std::collections::HashMap;
use crate::parser::ast::{Definition, Program, Qualifier, Struct};
use crate::parser::cerium_type::CeriumType;

impl Program {
    pub fn parse_structure(&self) -> (HashMap<Qualifier, Vec<(String, CeriumType)>>, HashMap<Qualifier, CeriumType>) {
        let mut structs = HashMap::new();
        let mut globals = HashMap::new();
        for definition in &self.definitions {
            match definition {
                Definition::Function(function) => {
                    let name = function.name.clone();
                    let params = function.parameters.iter().map(|(_, t)| t.to_owned()).collect();
                    let return_type = function.return_type.clone().map(|t| Box::new(t));
                    let function = CeriumType::Function(params, return_type);
                    globals.insert(name, function);
                },
                Definition::Struct(structure) => {
                    structs.insert(structure.name.clone(), structure.attributes.clone());
                },
            }
        }
        (structs, globals)
    }
}