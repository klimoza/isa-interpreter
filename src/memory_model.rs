use rand::seq::SliceRandom;

use crate::{threads::{SCThreadSystem, ThreadSystem, TSOThreadSystem, PSOThreadSystem}, storage::{SCStorageSystem, StorageSystem, TSOStorageSystem, PSOStorageSystem}, graph::Node, instruction::{Instruction, LabeledInstruction}};


pub trait MemoryModel {
  fn get_possible_executions(&self) -> Vec<Node>;
  fn random_step(&mut self, debug_print: bool);
  fn step(&mut self, node: Node, debug_print: bool);
}

pub struct SC {
  thread_system: SCThreadSystem,
  storage_system: SCStorageSystem
}

impl SC {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> SC {
    SC {
      thread_system: SCThreadSystem::new(instructions),
      storage_system: SCStorageSystem::new()
    }
  }
}

impl MemoryModel for SC {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.thread_system.get_possible_executions()
    }

    fn random_step(&mut self, debug_print: bool) {
      let executions = self.get_possible_executions();
      if executions.is_empty() {
        return;
      }
      let execution = executions.choose(&mut rand::thread_rng()).unwrap().clone();
      if debug_print {
        println!("{}: {:?}", execution.thread_id, execution.instruction);
      }
      self.step(execution, debug_print);
    }

    fn step(&mut self, node: Node, debug_print: bool) {
      self.thread_system.remove_node(&node);
      let thread_id = node.thread_id;
      let current_step = node.instruction.instruction;
      match current_step {
        Instruction::Const { r, value } => {
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::ArithPlus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value + r3_value);
        }
        Instruction::ArithMinus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value - r3_value);
        }
        Instruction::ArithMul { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value * r3_value);
        }
        Instruction::ArithDiv { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value / r3_value);
        }
        Instruction::Cond { r, label } => {
          let value = self.thread_system.get_register(thread_id, r);
          if value != 0 {
            self.thread_system.goto(label);
          }
        }
        Instruction::Load { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.storage_system.load(thread_id, address_value);
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::Store { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.thread_system.get_register(thread_id, r);
          self.storage_system.store(thread_id, address_value, value);
        }
        Instruction::Cas { mode: _, address, to, exp, des } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let exp_value = self.thread_system.get_register(thread_id, exp);
          let des_value = self.thread_system.get_register(thread_id, des);
          let value = self.storage_system.cas(thread_id, address_value, exp_value, des_value);
          self.thread_system.assign_register(thread_id, to, value);
        }
        Instruction::Fai { mode: _, address, to, inc } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let inc_value = self.thread_system.get_register(thread_id, inc);
          let value = self.storage_system.fai(thread_id, address_value, inc_value);
          self.thread_system.assign_register(thread_id, to, value);
        }
        Instruction::Fence { mode: _ } => {}
        Instruction::Propagate { thread_id: _, address: _, value: _ } => {}
      };
      if debug_print {
        print!("{:?}", self.thread_system);
        print!("{:?}\n", self.storage_system);
      }
    }
}

pub struct TSO {
  thread_system: TSOThreadSystem,
  storage_system: TSOStorageSystem
}

impl TSO {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> TSO {
    TSO {
      storage_system: TSOStorageSystem::new(instructions.len()),
      thread_system: TSOThreadSystem::new(instructions)
    }
  }
}

impl MemoryModel for TSO {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.thread_system.get_possible_executions()
    }

    fn random_step(&mut self, debug_print: bool) {
      let executions = self.get_possible_executions();
      if executions.is_empty() {
        return;
      }
      let execution = executions.choose(&mut rand::thread_rng()).unwrap().clone();
      if debug_print {
        println!("{}: {:?}", execution.thread_id, execution.instruction);
      }
      self.step(execution, debug_print);
    }

    fn step(&mut self, node: Node, debug_print: bool) {
      self.thread_system.remove_node(&node);
      let thread_id = node.thread_id;
      let current_step = node.instruction.instruction;
      match current_step {
        Instruction::Const { r, value } => {
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::ArithPlus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value + r3_value);
        }
        Instruction::ArithMinus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value - r3_value);
        }
        Instruction::ArithMul { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value * r3_value);
        }
        Instruction::ArithDiv { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value / r3_value);
        }
        Instruction::Cond { r, label } => {
          let value = self.thread_system.get_register(thread_id, r);
          if value != 0 {
            self.thread_system.goto(label);
          }
        }
        Instruction::Load { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.storage_system.load(thread_id, address_value);
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::Store { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.thread_system.get_register(thread_id, r);
          self.storage_system.store(thread_id, address_value, value);
          self.thread_system.add_propagate_node(thread_id, address_value, value);
        }
        Instruction::Cas { mode: _, address, to, exp, des } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let exp_value = self.thread_system.get_register(thread_id, exp);
          let des_value = self.thread_system.get_register(thread_id, des);
          let value = self.storage_system.cas(thread_id, address_value, exp_value, des_value);
          if value == exp_value {
            self.thread_system.add_propagate_node(thread_id, address_value, des_value);
          }
          self.thread_system.assign_register(thread_id, to, value);
        }
        Instruction::Fai { mode: _, address, to, inc } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let inc_value = self.thread_system.get_register(thread_id, inc);
          let value = self.storage_system.fai(thread_id, address_value, inc_value);
          self.thread_system.assign_register(thread_id, to, value);
          self.thread_system.add_propagate_node(thread_id, address_value, value + inc_value);
        }
        Instruction::Fence { mode: _ } => {}
        Instruction::Propagate { thread_id, address, value: _ } => {
          self.storage_system.propagate(thread_id, address);
        }
      }
      if debug_print {
        print!("{:?}", self.thread_system);
        print!("{:?}\n", self.storage_system);
      }
    }
}

pub struct PSO {
  thread_system: PSOThreadSystem,
  storage_system: PSOStorageSystem
}

impl PSO {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> PSO {
    PSO {
      storage_system: PSOStorageSystem::new(instructions.len()),
      thread_system: PSOThreadSystem::new(instructions)
    }
  }
}

impl MemoryModel for PSO {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.thread_system.get_possible_executions()
    }

    fn random_step(&mut self, debug_print: bool) {
      let executions = self.get_possible_executions();
      if executions.is_empty() {
        return;
      }
      let execution = executions.choose(&mut rand::thread_rng()).unwrap().clone();
      if debug_print {
        println!("{}: {:?}", execution.thread_id, execution.instruction);
      }
      self.step(execution, debug_print);
    }

    fn step(&mut self, node: Node, debug_print: bool) {
      self.thread_system.remove_node(&node);
      let thread_id = node.thread_id;
      let current_step = node.instruction.instruction;
      match current_step {
        Instruction::Const { r, value } => {
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::ArithPlus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value + r3_value);
        }
        Instruction::ArithMinus { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value - r3_value);
        }
        Instruction::ArithMul { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value * r3_value);
        }
        Instruction::ArithDiv { r1, r2, r3 } => {
          let r2_value = self.thread_system.get_register(thread_id, r2);
          let r3_value = self.thread_system.get_register(thread_id, r3);
          self.thread_system.assign_register(thread_id, r1, r2_value / r3_value);
        }
        Instruction::Cond { r, label } => {
          let value = self.thread_system.get_register(thread_id, r);
          if value != 0 {
            self.thread_system.goto(label);
          }
        }
        Instruction::Load { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.storage_system.load(thread_id, address_value);
          self.thread_system.assign_register(thread_id, r, value);
        }
        Instruction::Store { mode: _, address, r } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let value = self.thread_system.get_register(thread_id, r);
          self.storage_system.store(thread_id, address_value, value);
          self.thread_system.add_propagate_node(thread_id, address_value, value);
        }
        Instruction::Cas { mode: _, address, to, exp, des } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let exp_value = self.thread_system.get_register(thread_id, exp);
          let des_value = self.thread_system.get_register(thread_id, des);
          let value = self.storage_system.cas(thread_id, address_value, exp_value, des_value);
          if value == exp_value {
            self.thread_system.add_propagate_node(thread_id, address_value, des_value);
          }
          self.thread_system.assign_register(thread_id, to, value);
        }
        Instruction::Fai { mode: _, address, to, inc } => {
          let address_value = self.thread_system.get_register(thread_id, address);
          let inc_value = self.thread_system.get_register(thread_id, inc);
          let value = self.storage_system.fai(thread_id, address_value, inc_value);
          self.thread_system.assign_register(thread_id, to, value);
          self.thread_system.add_propagate_node(thread_id, address_value, value + inc_value);
        }
        Instruction::Fence { mode: _ } => {}
        Instruction::Propagate { thread_id, address, value: _ } => {
          self.storage_system.propagate(thread_id, address);
        }
      }
      if debug_print {
        print!("{:?}", self.thread_system);
        print!("{:?}\n", self.storage_system);
      }
    }
}

pub enum MemoryModelType {
  SC, // Sequential Consistency
  TSO, // Total Store Order
  PSO, // Partial Store Order
}