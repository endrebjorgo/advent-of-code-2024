use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
struct Computer {
    ip: usize,
    program: Vec<usize>,
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
        let program: Vec<usize> = program_str.split(",")
            .map(|e| usize::from_str_radix(e, 10).unwrap()).collect();
        
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
            let optype = self.program[self.ip];
            let operand = self.program[self.ip + 1];

            match optype {
                0 => {
                    self.a_reg /= usize::pow(2, self.combo(operand) as u32);
                },
                1 => {
                    self.b_reg ^= operand;
                },
                2 => {
                    self.b_reg = self.combo(operand) % 8;
                },
                3 => {
                    if self.a_reg != 0 {
                        self.ip = operand;
                        continue;
                    }
                },
                4 => {
                    self.b_reg ^= self.c_reg;
                },
                5 => {
                    output.push(self.combo(operand) % 8);
                },
                6 => {
                    self.b_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
                },
                7 => {
                    self.c_reg = self.a_reg / usize::pow(2, self.combo(operand) as u32);
                },
                _ => unreachable!(),
            }
            self.ip += 2;
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
