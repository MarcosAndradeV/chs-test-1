use crate::instructions::{Instr, InstrKind};

pub fn assemble(source: String) -> Vec<Instr> {
    source
    .lines()
    .map(|line| line.split_whitespace())
    .map(|mut sl| {
        match sl.next() {
            Some(s) => {
                match s {
                    "push" => {
                        let value = match sl.next() {
                            Some(v) => v.parse::<i64>().unwrap(),
                            None => todo!(),
                        };

                        return Instr::new(InstrKind::Push, Some(value))
                    },
                    _ => return Instr::new(InstrKind::Nop, None),
                }
            },
            None => return Instr::new(InstrKind::Nop, None),
        } 
    })
    .filter(|x| x.kind != InstrKind::Nop)
    .collect()
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1 () {
        let source = String::from("\n\n");
        let res = assemble(source);
        println!("Instr: {:?}", res);
        assert!(true);
    }
}