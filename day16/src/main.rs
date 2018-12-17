#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use nom::{digit, space};
use std::collections::*;
use std::io::prelude::*;

#[macro_use]
mod macros {
    macro_rules! behavior {
        ($name: ident $op:ident $result:path) => {
            fn $name(
                initial_state: Registers,
                instruction: &Instruction,
                expected: &Registers,
            ) -> Option<Operation> {
                let mut state = State::new(initial_state);
                if state.$op(instruction[1], instruction[2], instruction[3]).is_err() {
                    return None;
                }
                if &state.registers == expected {
                    Some($result)
                } else {
                    None
                }
            }
        };
    }
}

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;

    let mut file = File::open("input-day16")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let input: &str = &buf;
    let (rest, tests) = parse_tests(CompleteStr::from(input)).unwrap();
    let (_, instructions) = parse_instructions(rest).unwrap();
    println!("instructions_len: {}", instructions.len());

    println!("day16.1 {}", part1(&tests));
    println!("day16.2 {}", part2(&tests, &instructions));
    Ok(())
}

fn part1(input: &[InstructionTest]) -> usize {
    input.iter().map(|(start, inst, expected)| behavior_match(start, inst, expected).len()).filter(|x| x >= &3).count()        
}

fn part2(tests: &[InstructionTest], instructions: &[Instruction]) -> usize {    
    let opcodes = derive_opcodes(tests);
    execute_instructions(&opcodes, instructions).unwrap()
}


fn derive_opcodes(tests: &[InstructionTest]) -> HashMap<u8, Operation> {
    let potential_opcodes: HashMap<u8, HashSet<Operation>> = tests.iter().map(|(start, inst, expected)| {
        (inst[0], behavior_match(start, inst, expected))
    }).fold(HashMap::new(), |mut acc, (code, options)| {
        let options: HashSet<Operation> = options.iter().cloned().collect();
        acc.entry(code).and_modify(|e| *e = e.intersection(&options).cloned().collect()).or_insert(options);
        acc
    });

    let mut opcodes = HashMap::new();
    let mut found_ops = HashSet::new();
    while opcodes.len() < 16 {
        for (code, opset) in potential_opcodes.iter() {
            if opcodes.contains_key(code) {
                continue;
            }
            
            let mut set = opset.iter().filter(|op| !found_ops.contains(op));
            let first = set.next().unwrap();
            let second = set.next();
            if second.is_none() {
                opcodes.insert(*code, first.clone());
                found_ops.insert(first);
            }
        }
    }

    opcodes
}

fn execute_instructions(opcodes: &HashMap<u8, Operation>, instructions: &[Instruction]) -> Option<usize> {
    let mut state = State::new([0,0,0,0]);
    for &[op,a,b,c] in instructions {
        if state.exec(&opcodes[&op], a,b,c).is_err() {
            return None
        }
    }
    Some(state.register(0).unwrap())
}

struct State {
    registers: Registers,
}

impl State {
    fn new(registers: Registers) -> State {
        State {
            registers: registers,
        }
    }

    #[inline]
    fn register(&self, n: usize) -> Result<usize, ErrInvalidRegisterRange> {
        if n < 4 {
            Ok(self.registers[n])
        } else {
            Err(ErrInvalidRegisterRange(n))
        }
    }

    #[inline]    
    fn set_register(&mut self, n: usize, v: usize) -> StateResult{
        if n < 4{
            self.registers[n] = v;
            Ok(())
        } else {
            Err(ErrInvalidRegisterRange(n))
        }        
    }

    fn addr(&mut self, a: u8, b: u8, c: u8) -> StateResult{ 
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;
        let accum = va + vb;
        self.set_register(c as usize, accum)
    }

    fn addi(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;
        let accum = va + vb;
        self.set_register(c as usize, accum)
    }

    fn mulr(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;
        let accum = va * vb;
        self.set_register(c as usize, accum)
    }

    fn muli(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;
        let accum = va * vb;
        self.set_register(c as usize, accum)
    }

    fn banr(&mut self, a: u8, b: u8, c: u8) -> StateResult {
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;
        let accum = va & vb;
        self.set_register(c as usize, accum)
    }

    fn bani(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;
        let accum = va & vb;
        self.set_register(c as usize, accum)
    }

    fn borr(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;
        let accum = va | vb;
        self.set_register(c as usize, accum)
    }

    fn bori(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;
        let accum = va | vb;
        self.set_register(c as usize, accum)
    }

    fn setr(&mut self, a: u8, _b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        self.set_register(c as usize, va)
    }

    fn seti(&mut self, a: u8, _b: u8, c: u8) -> StateResult{
        let va = a as usize;
        self.set_register(c as usize, va)
    }

    fn gtir(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = a as usize;
        let vb = self.register(b as usize)?;

        let accum = if va > vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn gtri(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;

        let accum = if va > vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn gtrr(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;

        let accum = if va > vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn eqir(&mut self, a: u8, b: u8, c: u8) -> StateResult {
        let va = a as usize;
        let vb = self.register(b as usize)?;

        let accum = if va == vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn eqri(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = b as usize;

        let accum = if va == vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn eqrr(&mut self, a: u8, b: u8, c: u8) -> StateResult{
        let va = self.register(a as usize)?;
        let vb = self.register(b as usize)?;

        let accum = if va == vb { 1 } else { 0 };
        self.set_register(c as usize, accum)
    }

    fn exec(&mut self, op: &Operation, a: u8, b:u8, c:u8) -> StateResult {
        use crate::Operation::*;
        match op {
            Addr => self.addr(a,b,c),
            Addi => self.addi(a,b,c),
            Mulr => self.mulr(a,b,c),
            Muli => self.muli(a,b,c),
            Banr => self.banr(a,b,c),
            Bani => self.bani(a,b,c),
            Borr => self.borr(a,b,c),
            Bori => self.bori(a,b,c),
            Setr => self.setr(a,b,c),
            Seti => self.seti(a,b,c),
            Gtir => self.gtir(a,b,c),
            Gtri => self.gtri(a,b,c),
            Gtrr => self.gtrr(a,b,c),
            Eqir => self.eqir(a,b,c),
            Eqri => self.eqri(a,b,c),
            Eqrr => self.eqrr(a,b,c),
        }
    }
}

type StateResult = Result<(), ErrInvalidRegisterRange>;

#[derive(Hash, PartialEq, Debug, Eq, Clone)]
enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

type Instruction = [u8; 4];
type Registers = [usize; 4];

#[derive(Debug)]
struct ErrInvalidRegisterRange(usize);

impl std::fmt::Display for ErrInvalidRegisterRange{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Register out of range {}", self.0)
    }
}

fn behavior_match(start: &Registers, inst: &Instruction, expected: &Registers) -> Vec<Operation> {
    let mut out = Vec::new();

    if let Some(op) = test_addr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_addi(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_mulr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_muli(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_banr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_bani(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_borr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_bori(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_setr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_seti(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_gtir(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_gtri(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_gtrr(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_eqir(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_eqri(start.clone(), inst, expected) {
        out.push(op);
    }
    if let Some(op) = test_eqrr(start.clone(), inst, expected) {
        out.push(op);
    }
    out
}

behavior!(test_addr addr Operation::Addr);
behavior!(test_addi addi Operation::Addi);
behavior!(test_mulr mulr Operation::Mulr);
behavior!(test_muli muli Operation::Muli);
behavior!(test_banr banr Operation::Banr);
behavior!(test_bani bani Operation::Bani);
behavior!(test_borr borr Operation::Borr);
behavior!(test_bori bori Operation::Bori);
behavior!(test_setr setr Operation::Setr);
behavior!(test_seti seti Operation::Seti);
behavior!(test_gtir gtir Operation::Gtir);
behavior!(test_gtri gtri Operation::Gtri);
behavior!(test_gtrr gtrr Operation::Gtrr);
behavior!(test_eqir eqir Operation::Eqir);
behavior!(test_eqri eqri Operation::Eqri);
behavior!(test_eqrr eqrr Operation::Eqrr);

named!(parse_before<CompleteStr, Registers>,
       do_parse!(
           tag!("Before: ") >>
               registers: parse_registers >>
               (registers)           
       ));

named!(parse_instruction<CompleteStr, Instruction>,
       do_parse!(
           one: digit >>
               space >>
               two: digit >>
               space >>
               three: digit >>
               space >>
               four: digit >>
               alt!(tag!("\n")|eof!()) >>
               ([one.as_ref().parse().unwrap(),
                 two.as_ref().parse().unwrap(),
                 three.as_ref().parse().unwrap(),
                 four.as_ref().parse().unwrap(),
                 ])
               ));
           
named!(parse_registers<CompleteStr, Registers>,
       do_parse!(
           tag!("[") >>
               one: digit >>
               tag!(", ") >>
               two: digit >>
               tag!(", ") >>
               three: digit >>
               tag!(", ") >>
               four: digit >>
               tag!("]") >>
               ([one.as_ref().parse().unwrap(),
                 two.as_ref().parse().unwrap(),
                 three.as_ref().parse().unwrap(),
                 four.as_ref().parse().unwrap(),
               ])
       ));

named!(parse_after<CompleteStr, Registers>,
       do_parse!(
           tag!("After:  ") >>
               registers: parse_registers >>
               (registers)
       ));

named!(parse_test<CompleteStr, InstructionTest>,
       do_parse!(
           before: parse_before >>
               tag!("\n") >>
               instruction: parse_instruction >>
               after: parse_after >>
               (before, instruction, after)
       ));

type InstructionTest = (Registers, Instruction, Registers);

named!(parse_tests<CompleteStr, Vec<InstructionTest>>,
       many1!(
           ws!(parse_test)));

named!(parse_instructions<CompleteStr, Vec<Instruction>>,
       many1!(
           ws!(parse_instruction)));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_test() {
        let buf = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#.into();
        assert_eq!(Ok(("".into(), ([3,2,1,1], [9,2,1,2], [3,2,2,1]))), parse_test(buf));
    }

    #[test]
    fn test_parse_before() {
        let buf = r#"Before: [3, 2, 1, 1]"#.into();
        assert_eq!(Ok(("".into(), [3,2,1,1])), parse_before(buf));
    }

    #[test]
    fn test_parse_after() {
        let buf = r#"After:  [3, 2, 2, 1]"#.into();
        assert_eq!(Ok(("".into(), [3,2,2,1])), parse_after(buf));
    }

    #[test]
    fn test_parse_instruction() {
        let buf = r#"9 2 1 2"#.into();
        assert_eq!(Ok(("".into(), [9,2,1,2])), parse_instruction(buf));
    }

    #[test]
    fn test_parse_tests() {
        let buf = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]

Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
"#.into();
        assert_eq!(Ok(("".into(),
                       vec![
                           ([3,2,1,1], [9,2,1,2], [3,2,2,1]),
                           ([3,2,1,1], [9,2,1,2], [3,2,2,1]),
                       ])
        ), parse_tests(buf));
    }
}
