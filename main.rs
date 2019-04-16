
type RegisterFile = [u16; 3];

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Halt,
    Load { reg: usize, value: u16 },
    Swap { reg1: usize, reg2: usize, reg3: usize },
    Add { reg1: usize, reg2: usize, reg3: usize },
    Branch { offset: usize }
}

impl Instruction {
    fn decode(encoded_instruction: u16) -> Option<Self> {
        let operator = encoded_instruction >> 12;
        let reg1 = ((encoded_instruction >> 8) & 0xF) as usize;
        let reg2 = ((encoded_instruction >> 4) & 0xF) as usize;
        let reg3 = (encoded_instruction & 0xF) as usize;
        let offset = (encoded_instruction & 0xFFF) as usize;
        let value = encoded_instruction & 0xFF;

        match operator {
            0 => Some(Instruction::Halt),
            1 => Some(Instruction::Load { reg: reg1, value: value }),
            2 => Some(Instruction::Swap { reg1: reg1, reg2: reg2, reg3: reg3 }),
            3 => Some(Instruction::Add { reg1: reg1, reg2: reg2, reg3: reg3 }),
            4 => Some(Instruction::Branch { offset: offset }),
            _ => None,
        }
    }

    fn execute(&self, registers: &mut [u16], pc: &mut usize) -> bool {
        match *self {
            Instruction::Load { reg, value } => {
                load(reg, value, registers);
            },
            Instruction::Swap { reg1, reg2, reg3 } => {
                swap(reg1, reg2, reg3, registers);
            },
            Instruction::Add { reg1, reg2, reg3 } => {
                add(reg1, reg2, reg3, registers);
            },
            Instruction::Branch { offset } => {
                branch(offset, pc);
            },
            Instruction::Halt => {
                halt(registers);
                return false;
            },
        }

        true
    }
}

fn halt(register_file: &[u16]) {
    println!("{:?}", register_file[0]);
}

fn load(register: usize, value: u16, register_file: &mut [u16]) {
    register_file[register] = value;
}

fn swap(reg1: usize, reg2: usize, reg3: usize, register_file: &mut [u16]) {
    register_file[reg3] = register_file[reg1];
    register_file[reg1] = register_file[reg2];
    register_file[reg2] = register_file[reg3];
}

fn add(reg1: usize, reg2: usize, reg3: usize, register_file: &mut [u16]) {
    register_file[reg3] = register_file[reg1] + register_file[reg2];
}

fn branch(offset: usize, pc: &mut usize) {
    *pc -= offset - 1;
}

struct Program<'a> {
    instructions: &'a [u16],
}

impl<'a> Program<'a> {
    fn fetch(&self, pc: usize) -> u16 {
        self.instructions[pc]
    }
}

fn cpu(program: Program) {
    let mut pc = 0;
    let mut registers = RegisterFile::default();

    loop {
        let encoded_instruction = program.fetch(pc);
        let decoded_instruction = Instruction::decode(encoded_instruction);

        match decoded_instruction {
            Some(instr) => {
                if !instr.execute(&mut registers, &mut pc) { break }
            }
            None => break,
        }

        pc += 1;
    }
}

fn main() {
    let encoded_instructions = Program { instructions: &[0x1110, 0x2100, 0x3010, 0x0] };

    cpu(encoded_instructions);
}
