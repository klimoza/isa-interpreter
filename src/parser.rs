use std::str::FromStr;

use crate::instruction::{Mode, LabeledInstruction, Instruction};

impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "SEQ_CST" => Ok(Mode::SeqCst),
            "REL" => Ok(Mode::Rel),
            "ACQ" => Ok(Mode::Acq),
            "REL_ACQ" => Ok(Mode::RelAcq),
            "RLX" => Ok(Mode::Rlx),
            _ => Err(()),
        }
    }
}

pub fn parse_instruction(line: &str) -> Result<LabeledInstruction, String> {
    let mut parts: Vec<&str> = line.split_whitespace().collect();
  
    let label: Option<String> = 
        if parts[0].ends_with(":") {
            Some(parts[0].to_string().replace(":", ""))
        } else {
            None
        };
    
    if label.is_some() {
      parts.remove(0);
    }

    let instruction: Instruction = match parts.as_slice() {
        [r, "=", value] => {
            let value: i32 = value.parse().map_err(|_| "Invalid constant".to_string())?;
            Instruction::Const { r: r.to_string(), value }
        },
        [r1, "=", r2, "+", r3] => Instruction::ArithPlus { r1: r1.to_string(), r2: r2.to_string(), r3: r3.to_string() },
        [r1, "=", r2, "-", r3] => Instruction::ArithMinus { r1: r1.to_string(), r2: r2.to_string(), r3: r3.to_string() },
        [r1, "=", r2, "*", r3] => Instruction::ArithMul { r1: r1.to_string(), r2: r2.to_string(), r3: r3.to_string() },
        [r1, "=", r2, "/", r3] => Instruction::ArithDiv { r1: r1.to_string(), r2: r2.to_string(), r3: r3.to_string() },
        ["load", mode, address, r] => {
            let mode: Mode = mode.parse().map_err(|_| "Invalid mode".to_string())?;
            Instruction::Load { mode, address: address.to_string(), r: r.to_string() }
        },
        ["store", mode, address, r] => {
            let mode: Mode = mode.parse().map_err(|_| "Invalid mode".to_string())?;
            Instruction::Store { mode, address: address.to_string(), r: r.to_string() }
        },
        [to, ":=", "cas", mode, address, exp, des] => {
            let mode: Mode = mode.parse().map_err(|_| "Invalid mode".to_string())?;
            Instruction::Cas { mode, address: address.to_string(), to: to.to_string(), exp: exp.to_string(), des: des.to_string() }
        },
        [to, ":=", "fai", mode, address, inc] => {
            let mode: Mode = mode.parse().map_err(|_| "Invalid mode".to_string())?;
            Instruction::Fai { mode, address: address.to_string(), to: to.to_string(), inc: inc.to_string() }
        },
        ["fence", mode] => {
            let mode: Mode = mode.parse().map_err(|_| "Invalid mode".to_string())?;
            Instruction::Fence { mode }
        },
        ["if", r, "goto", label] => Instruction::Cond { r: r.to_string(), label: label.to_string() },
        _ => return Err("Unknown instruction format".to_string()),
    };

    Ok(LabeledInstruction {
        label,
        instruction,
    })
}
