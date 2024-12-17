use std::fs;

#[derive(Debug)]
#[repr(usize)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Debug)]
struct Computer {
    registers: [u64; 3],
    instruction_pointer: usize,
    output: Vec<u8>,
    program: Vec<u8>,
}

impl Computer {
    fn from_input(input: &str) -> Computer {
        let (register_input, program_input) = input.split_once("\n\n").unwrap();

        let registers = register_input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u64>>();

        let program = program_input
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Computer {
            registers: [registers[0], registers[1], registers[2]],
            instruction_pointer: 0,
            output: Vec::new(),
            program,
        }
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4..=6 => self.registers[(operand - 4) as usize] as u64,
            _ => panic!(),
        }
    }

    fn dv(&mut self, register: Register, operand: u8) -> usize {
        let numerator = self.registers[Register::A as usize];
        self.registers[register as usize] = numerator >> self.combo_operand(operand);
        self.instruction_pointer + 2
    }

    fn adv(&mut self, operand: u8) -> usize {
        self.dv(Register::A, operand)
    }

    fn bdv(&mut self, operand: u8) -> usize {
        self.dv(Register::B, operand)
    }

    fn cdv(&mut self, operand: u8) -> usize {
        self.dv(Register::C, operand)
    }

    fn bxl(&mut self, operand: u8) -> usize {
        self.registers[Register::B as usize] =
            self.registers[Register::B as usize] ^ operand as u64;
        self.instruction_pointer + 2
    }

    fn bxc(&mut self, _operand: u8) -> usize {
        self.registers[Register::B as usize] =
            self.registers[Register::B as usize] ^ self.registers[Register::C as usize];
        self.instruction_pointer + 2
    }

    fn bst(&mut self, operand: u8) -> usize {
        self.registers[Register::B as usize] = self.combo_operand(operand) as u64 % 8;
        self.instruction_pointer + 2
    }

    fn jnz(&mut self, operand: u8) -> usize {
        if self.registers[Register::A as usize] == 0 {
            self.instruction_pointer + 2
        } else {
            operand as usize
        }
    }

    fn out(&mut self, operand: u8) -> usize {
        self.output.push((self.combo_operand(operand) % 8) as u8);
        self.instruction_pointer + 2
    }

    fn execute_instruction(&mut self) {
        let opcode = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        self.instruction_pointer = match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid opcode"),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.execute_instruction();
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn run(input: &str) {
    let mut computer = Computer::from_input(input);

    computer.run();

    println!("Output simulation: {}", computer.get_output());

    /*
    Program: 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0

    2,4  bst 4  B=A%8
    1,5  bxl 5  B=B^5
    7,5  cdv 5  C=A>>B
    4,3  bxc 3  B=B^C
    1,6  bxl 6  B=B^6
    0,3  adv 3  A=A>>3
    5,5  out 5  output.push(B%8)
    3,0  jnz 0  if A==0 { Stop } else { Restart }
    */

    fn find_a_register(sequence: &[u64], a: u64) -> Option<u64> {
        let new_b = match sequence.last() {
            Some(b) => b,
            None => return Some(a),
        };
        for new_a_bits in 0..8 {
            let tmp_a = (a << 3) | new_a_bits;

            let mut tmp_b = tmp_a % 8;
            tmp_b = tmp_b ^ 5;
            let tmp_c = tmp_a >> tmp_b;
            tmp_b = tmp_b ^ tmp_c;
            tmp_b = tmp_b ^ 6;

            if tmp_b % 8 == *new_b {
                let a = (a << 3) | new_a_bits;
                let t = find_a_register(&sequence[0..sequence.len() - 1], a);
                if t.is_some() {
                    return t;
                }
            }
        }
        return None;
    }
    let program = [2, 4, 1, 5, 7, 5, 4, 3, 1, 6, 0, 3, 5, 5, 3, 0];
    let mut a = find_a_register(&program, 0).unwrap();

    println!("Initial register A: {}", a);

    let mut output = Vec::new();
    loop {
        let b = a % 8;
        let b = b ^ 5;
        let c = a >> b;
        let b = b ^ c;
        let b = b ^ 6;
        a = a >> 3;
        output.push(b % 8);
        if a == 0 {
            break;
        }
    }
    println!(
        "Output directly:   {}",
        output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    println!(
        "Output target:     {}",
        program
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    run(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Run with cargo test -- --nocapture
    #[test]
    fn sample_input_1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        run(&input);
    }

    #[test]
    fn sample_input_2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        run(&input);
    }

    #[test]
    fn test_1() {
        let input = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";

        let mut computer = Computer::from_input(input);
        computer.run();
        assert!(computer.registers[Register::B as usize] == 1);
    }

    #[test]
    fn test_2() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

        let mut computer = Computer::from_input(input);
        computer.run();
        assert!(computer.get_output().as_str() == "0,1,2");
    }

    #[test]
    fn test_3() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let mut computer = Computer::from_input(input);
        computer.run();
        assert!(computer.get_output().as_str() == "4,2,5,6,7,7,7,7,3,1,0");
        assert!(computer.registers[Register::A as usize] == 0);
    }

    #[test]
    fn test_4() {
        let input = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";

        let mut computer = Computer::from_input(input);
        computer.run();
        assert!(computer.registers[Register::B as usize] == 26);
    }

    #[test]
    fn test_5() {
        let input = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";

        let mut computer = Computer::from_input(input);
        computer.run();
        assert!(computer.registers[Register::B as usize] == 44354);
    }
}
