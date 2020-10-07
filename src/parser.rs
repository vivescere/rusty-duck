/// Parse a '.duck' file to a Program struct
pub fn parse(source: &str) -> Option<Program> {
    Program::from_source(source)
}

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Alt(String),
    Control(String),
    ControlAlt(String),
    ControlShift(String),
    DefaultDelay(u32),
    Delay(u32),
    GUI(String),
    REM(String),
    AltShift,
    Shift(String),
    EnterString(String),
    StringDelay(u32),
    Repeat(u32),
    KeyName(String),
}

impl Program {
    fn from_source(source: &str) -> Option<Self> {
        let mut instructions = vec![];

        for line in source.split("\n") {
            if line.trim().is_empty() {
                continue;
            }

            let instruction = Instruction::from_str(line);
            instructions.push(instruction?);
        }

        Some(Self { instructions })
    }
}

impl Instruction {
    fn from_str(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(" ").collect();

        match *parts.get(0)? {
            "DELAY" => Self::parse_delay(parts),
            "GUI" => Self::parse_gui(parts),
            "STRING" => Self::parse_string(parts),
            _ => Some(Self::KeyName(parts.get(0)?.to_string())),
        }
    }

    fn parse_delay(parts: Vec<&str>) -> Option<Self> {
        Some(Self::Delay(parts.get(1)?.parse().ok()?))
    }

    fn parse_gui(parts: Vec<&str>) -> Option<Self> {
        Some(Self::GUI(parts.get(1)?.to_string()))
    }

    fn parse_string(parts: Vec<&str>) -> Option<Self> {
        let string: String = parts
            .iter()
            .skip(1)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join(" ");

        Some(Self::EnterString(string))
    }
}
