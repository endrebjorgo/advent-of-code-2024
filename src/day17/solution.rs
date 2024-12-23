use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
enum OpType {
    ADV = 0,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl OpType {
    fn from_usize(value: usize) -> Option<OpType> {
        match value {
            0 => Some(OpType::ADV),
            1 => Some(OpType::BXL),
            2 => Some(OpType::BST),
            3 => Some(OpType::JNZ),
            4 => Some(OpType::BXC),
            5 => Some(OpType::OUT),
            6 => Some(OpType::BDV),
            7 => Some(OpType::CDV),
            _ => None,
        }
    }
}

type Program = Vec<(OpType, usize)>;

#[derive(Debug)]
struct Computer {
    ip: usize,
    program: Program,
    a_reg: usize,
    b_reg: usize,
    c_reg: usize,
}

impl Computer {
    fn from_file(file_path: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        let mut buf_lines = buf.lines();

        let a_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let b_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let c_reg_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;

        let a_reg = usize::from_str_radix(a_reg_str, 10).unwrap();
        let b_reg = usize::from_str_radix(b_reg_str, 10).unwrap();
        let c_reg = usize::from_str_radix(c_reg_str, 10).unwrap();

        assert!(buf_lines.next() == Some(""));

        let program_str = buf_lines.next().unwrap().split_once(": ").unwrap().1;
        let program_vec: Vec<usize> = program_str.split(",")
            .map(|e| usize::from_str_radix(e, 10).unwrap()).collect();
        
        let mut program = Program::new();
        let mut idx = 0;
        while idx < program_vec.len() {
            if let Some(optype) = OpType::from_usize(program_vec[idx]) {
                program.push((optype, program_vec[idx+1]));
            } else {
                panic!();
            }
            idx += 2;
        }

        assert!(buf_lines.next() == None);

        Self { ip: 0, program, a_reg, b_reg, c_reg }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            x if x <= 3 => x,
            4 => self.a_reg,
            5 => self.b_reg,
            6 => self.c_reg,
            _ => unreachable!(),
        }
    }

    fn execute_program(&mut self) -> Vec<usize> {
        let mut output: Vec<usize> = Vec::new();

        while self.ip < self.program.len() {
            match self.program[self.ip] {
                (OpType::ADV, operand) => {
                    self.a_reg /= usize::pow(2, self.combo(operand) as u32);
                },
                (OpType::BXL, operand) => {
                    self.b_reg ^= operand;
                },
                (OpType::BST, operand) => {
                    self.b_reg = self.combo(operand) % 8;
                },
                (OpType::JNZ, operand) => {
                    if self.a_reg != 0 {
                        assert!(operand % 2 == 0);
                        self.ip = operand / 2;
                        continue;
                    }
                },
                (OpType::BXC, _) => {
                    self.b_reg ^= self.c_reg;
                },
                (OpType::OUT, operand) => {
                    output.push(self.combo(operand) % 8);
                },
                (OpType::BDV, operand) => {
                    self.b_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
                },
                (OpType::CDV, operand) => {
                    self.c_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
                },
            }
            self.ip += 1;
        }
        self.ip = 0;
        output
    }
}

fn part1(file_path: &str) -> Vec<usize> {
    let mut c = Computer::from_file(file_path);
    c.execute_program()
}

fn main() {
    let mut start = Instant::now();

    let output = part1("src/day17/input.txt").iter()
        .map(|e| e.to_string()).collect::<Vec<String>>().join(",");
    println!(
        "Output: {} (Duration: {:.2?})",
        output,
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res: Vec<usize> = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        assert_eq!(res, part1("src/day17/test.txt"));
    }
}
