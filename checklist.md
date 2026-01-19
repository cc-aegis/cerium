# Compilable
```rust
trait Compilable {
    fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        warn!(...);
        self.compile(vars);
    }
    fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        warn!(...);
        self.compile_mut(vars); // or other way around where _mut per default allocates new memory
    }
    fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError>;
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError>;
}

impl Compilable for Integer {
    fn compile_unit(self, vars: &mut Vars) -> Result<Vec<Instruction>, CompilerError> {
        Ok(Vec::new())
    }

    fn compile(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        
        Ok((Vec::new(), Some((Operand::Const(self.value), CeriumType::U16))))
    }

    fn compile_mut(self, vars: &mut Vars) -> Result<(Vec<Instruction>, Option<(Operand, CeriumType)>), CompilerError> {
        let (address, alloc, dealloc) = vars.new();
        Ok((vec![
            Instruction::alloc(address),
            Instruction::Mov(address, Operand::Const(self.value)),
            Instruction::dealloc(address),
        ], Some((address, CeriumType::U16))))
    }
    
    fn compile_into(self, vars: &mut Vars, target: Operand) -> Result<(Vec<Instruction>, Option<CeriumType>), CompilerError> {
        Ok((vec![
            Instruction::Mov(target, Operand::Const(self.value)),
        ], Some(CeriumType::U16)))
    }
}


```

# Project Structure
- src
  - main 
  - error
    - ..
  - ast
    - mod
    - compilable
    - node (enum, wraps following types)
    - ..<thingy> (struct, implements compilable)
  - ir
    - ..<stuff for intermediate asm>
    - 
    
noch tokens, lexing, parsing und ir->asm

# IR

```
mov 
```