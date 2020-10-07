/// Parse a '.duck' file to a Program struct
pub fn parse(source: &str) -> Option<Program> {
    Program::from_source(source)
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Delay(u32),
}

impl Program {
    fn from_source(source: &str) -> Option<Self> {
        let mut instructions = vec![];

        for line in source.split("\n") {
            let instruction = Instruction::from_str(line)?;
            instructions.push(instruction);
        }

        Some(Self { instructions })
    }
}

impl Instruction {
    fn from_str(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(" ").collect();

        match *parts.get(0)? {
            "DELAY" => Self::parse_delay(parts),
            _ => None,
        }
    }

    fn parse_delay(parts: Vec<&str>) -> Option<Self> {
        Some(Self::Delay(parts.get(1)?.parse().ok()?))
    }
}
