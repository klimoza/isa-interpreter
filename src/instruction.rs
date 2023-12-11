use std::fmt::Debug;

#[derive(Clone, Copy)]
pub enum Mode {
  SeqCst,
  Rel,
  Acq,
  RelAcq,
  Rlx
}

impl Debug for Mode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Mode::SeqCst => write!(f, "SEQ_CST"),
      Mode::Rel => write!(f, "REL"),
      Mode::Acq => write!(f, "ACQ"),
      Mode::RelAcq => write!(f, "REL_ACQ"),
      Mode::Rlx => write!(f, "RLX")
    }
  }

}

#[derive(Clone)]
pub enum Instruction {
  Const { r: String, value: i32,  },
  ArithPlus { r1: String, r2: String, r3: String },
  ArithMinus { r1: String, r2: String, r3: String },
  ArithMul { r1: String, r2: String, r3: String },
  ArithDiv { r1: String, r2: String, r3: String },
  Cond { r: String, label: String },
  Load { mode: Mode, address: String, r: String },
  Store { mode: Mode, address: String, r: String },
  Cas { mode: Mode, address: String, to: String, exp: String, des: String },
  Fai { mode: Mode, address: String, to: String, inc: String },
  Fence { mode: Mode },
  Propagate { thread_id: usize, address: i32, value: i32 }
}

impl Debug for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instruction::Const { r, value } => write!(f, "{} = {}", r, value),
      Instruction::ArithPlus { r1, r2, r3 } => write!(f, "{} = {} + {}", r1, r2, r3),
      Instruction::ArithMinus { r1, r2, r3 } => write!(f, "{} = {} - {}", r1, r2, r3),
      Instruction::ArithMul { r1, r2, r3 } => write!(f, "{} = {} * {}", r1, r2, r3),
      Instruction::ArithDiv { r1, r2, r3 } => write!(f, "{} = {} / {}", r1, r2, r3),
      Instruction::Cond { r, label } => write!(f, "if {} goto {}", r, label),
      Instruction::Load { mode, address, r } => write!(f, "load {:?} #{} {}", mode, address, r),
      Instruction::Store { mode, address, r } => write!(f, "store {:?} #{} {}", mode, address, r),
      Instruction::Cas { mode, address, to, exp, des } => write!(f, "{} := cas {:?} #{} {} {}", to, mode, address, exp, des),
      Instruction::Fai { mode, address, to, inc } => write!(f, "{} := fai {:?} #{} {}", to, mode, address, inc),
      Instruction::Fence { mode } => write!(f, "fence {:?}", mode),
      Instruction::Propagate { thread_id, address, value } => write!(f, "propagate with thread_id = {}, address = {} and value = {}", thread_id, address, value)
    }
  }
}

#[derive(Clone)]
pub struct LabeledInstruction {
  pub label: Option<String>,
  pub instruction: Instruction
}

impl Debug for LabeledInstruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.label {
      Some(label) => write!(f, "{}: {:?}", label, self.instruction),
      None => write!(f, "{:?}", self.instruction)
    }
  }

}

impl LabeledInstruction {
  pub fn get_mode(&self) -> Option<Mode> {
    match self.instruction {
      Instruction::Const { r: _, value: _ } => None,
      Instruction::ArithPlus { r1: _, r2: _, r3: _ } => None,
      Instruction::ArithMinus { r1: _, r2: _, r3: _ } => None,
      Instruction::ArithMul { r1: _, r2: _, r3: _ } => None,
      Instruction::ArithDiv { r1: _, r2: _, r3: _ } => None,
      Instruction::Cond { r: _, label: _ } => None,
      Instruction::Load { mode, address: _, r: _ } => Some(mode),
      Instruction::Store { mode, address: _, r: _ } => Some(mode),
      Instruction::Cas { mode, address: _, to: _, exp: _, des: _ } => Some(mode),
      Instruction::Fai { mode, address: _, to: _, inc: _ } => Some(mode),
      Instruction::Fence { mode } => Some(mode),
      Instruction::Propagate { thread_id: _, address: _, value: _ } => None
    }
  }

  pub fn is_fence(&self) -> bool {
    match self.instruction {
      Instruction::Fence { mode: _ } => true,
      _ => false
    }
  }
}