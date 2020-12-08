#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    Acc(i16),
    Jmp(i16),
    Noop(i16),
}

struct GameVm {
    code: Vec<OpCode>,
}

impl GameVm {
    fn parse(s: &str) -> Vec<OpCode> {
        s.split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| (&line[0..3], &line[4..]))
            .map(|(op, arg)| {
                let iarg = arg.parse::<i16>().unwrap();
                match op {
                    "acc" => OpCode::Acc(iarg),
                    "jmp" => OpCode::Jmp(iarg),
                    "nop" => OpCode::Noop(iarg),
                    _ => unimplemented!(),
                }
            })
            .collect()
    }

    pub fn from(s: &str) -> Self {
        let code = Self::parse(s);
        Self { code }
    }

    pub fn exec(&mut self) -> Result<i64, i64> {
        let mut pc: usize = 0;
        let mut acc: i64 = 0;
        let mut trace: Vec<bool> = vec![false; self.code.len()];

        while pc < self.code.len() {
            let op = &self.code[pc];
            // println!("{:?} (pc: {}, acc: {})", op, pc, acc);
            if trace[pc] {
                return Err(acc);
            }

            trace[pc] = true;
            match op {
                OpCode::Acc(arg) => acc += *arg as i64,
                OpCode::Jmp(arg) => {
                    pc = (pc as i64 + *arg as i64) as usize;
                    continue;
                }
                OpCode::Noop(_) => (),
            }
            pc += 1;
        }

        Ok(acc)
    }

    pub fn code(&self) -> &[OpCode] {
        &self.code
    }

    pub fn with_modified(&self, i: usize, op: OpCode) -> Self {
        let mut code = self.code.clone();
        code[i] = op;
        Self { code }
    }
}

fn calc1(input: &str) -> i64 {
    GameVm::from(input).exec().unwrap_err()
}

fn calc2(input: &str) -> Option<i64> {
    let vm = GameVm::from(input);

    vm.code()
        .iter()
        .enumerate()
        .filter_map(|(i, op)| match op {
            OpCode::Acc(_) => None,
            OpCode::Noop(arg) => Some((i, OpCode::Jmp(*arg))),
            OpCode::Jmp(arg) => Some((i, OpCode::Noop(*arg))),
        })
        .filter_map(|(i, op)| vm.with_modified(i, op).exec().ok())
        .next()
}

fn main() {
    println!("{:?}", calc1(include_str!("aoc08.txt")));
    println!("{:?}", calc2(include_str!("aoc08.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(calc1(input), 5);
    }

    #[test]
    fn check2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(calc2(input), Some(8));
    }
}
