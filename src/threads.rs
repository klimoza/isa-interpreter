use std::collections::{HashMap, HashSet};
use core::fmt::Debug;
use crate::{graph::{Node, Graph}, instruction::{LabeledInstruction, self}};

pub trait ThreadSystem {
  fn get_possible_executions(&self) -> Vec<Node>;
  fn assign_register(&mut self, thread_id: usize, register: String, value: i32);
  fn get_register(&self, thread_id: usize, register: String) -> i32;
  fn remove_node(&mut self, node: &Node);
  fn goto(&mut self, label: String);
}

pub struct SCThreadSystem {
  graph: Graph,
  registers: Vec<HashMap<String, i32>>
}

impl Debug for SCThreadSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# REGISTERS\n")?;
    for (i, register) in self.registers.iter().enumerate() {
      write!(f, "| Thread {}: {:?}\n", i, register)?;
    }
    Ok(())
  }
}

impl SCThreadSystem {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> SCThreadSystem {
    let mut graph = Graph::new();
    let mut registers = Vec::new();
    for _ in 0..instructions.len() {
      registers.push(HashMap::new());
    }
    for (thread_id, thread_instructions) in instructions.iter().enumerate() {
      let mut instruction_ids: Vec<usize> = Vec::new();
      for instruction in thread_instructions.iter() {
        let id = graph.add_node(thread_id as usize, instruction.clone());
        for previous_instruction in instruction_ids.iter() {
          graph.add_edge(id, *previous_instruction);
        }
        instruction_ids.push(id);
      }
    }
    SCThreadSystem {
      graph,
      registers
    }
  }
}

impl ThreadSystem for SCThreadSystem {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.graph.execution_candidates.iter().map(|id| self.graph.instructions[*id].clone()).collect()
    }

    fn assign_register(&mut self, thread_id: usize, register: String, value: i32) {
      self.registers[thread_id].insert(register, value);
    }

    fn get_register(&self, thread_id: usize, register: String) -> i32 {
      match self.registers[thread_id].get(&register) {
        Some(value) => *value,
        None => 0
      }
    }

    fn remove_node(&mut self, node: &Node) {
      self.graph.remove_node(node.id);
    }

    fn goto(&mut self, label: String) {
      if !self.graph.is_label_active(label.clone()) {
        let mut current_label: Option<String> = None;
        while current_label != Some(label.clone()) {
          current_label = self.graph.restore_node().clone();
        }
      }
    }
}

pub struct TSOThreadSystem {
  graph: Graph,
  registers: Vec<HashMap<String, i32>>,
  propagate_nodes: Vec<HashSet<usize>>
}

impl Debug for TSOThreadSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# REGISTERS\n")?;
    for (i, register) in self.registers.iter().enumerate() {
      write!(f, "| Thread {}: {:?}\n", i, register)?;
    }
    Ok(())
  }
}

impl TSOThreadSystem {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> TSOThreadSystem {
    let mut graph = Graph::new();
    let mut registers = Vec::new();
    let mut propagate_nodes = Vec::new();
    for _ in 0..instructions.len() {
      registers.push(HashMap::new());
      propagate_nodes.push(HashSet::new());
    }
    for (thread_id, thread_instructions) in instructions.iter().enumerate() {
      let mut instruction_ids: Vec<usize> = Vec::new();
      for instruction in thread_instructions.iter() {
        let id = graph.add_node(thread_id as usize, instruction.clone());
        instruction_ids.push(id);
      }
      for (i, instruction) in thread_instructions.iter().enumerate() {
        match instruction.get_mode() {
          Some(instruction::Mode::Rel) => {
            for j in i + 1..thread_instructions.len() {
              graph.add_edge(instruction_ids[j], instruction_ids[i]);
            }
          }
          Some(instruction::Mode::Acq) => {
            for j in 0..i {
              graph.add_edge(instruction_ids[i], instruction_ids[j]);
            }
          }
          Some(instruction::Mode::RelAcq) => {
            for j in 0..i {
              graph.add_edge(instruction_ids[i], instruction_ids[j]);
            }
            for j in i + 1..thread_instructions.len() {
              graph.add_edge(instruction_ids[j], instruction_ids[i]);
            }
          }
          Some(instruction::Mode::SeqCst) => {}
          Some(instruction::Mode::Rlx) => {}
          None => {}
        }
      }
    }
    TSOThreadSystem {
      graph,
      registers,
      propagate_nodes
    }
  }

  pub fn add_propagate_node(&mut self, thread_id: usize, address: i32, value: i32) {
    let id = self.graph.add_node(thread_id, LabeledInstruction {
      label: None,
      instruction: instruction::Instruction::Propagate { thread_id, address, value }
    });
    let active_fence_nodes = self.graph.active_fence_nodes.clone();
    for node in active_fence_nodes {
      self.graph.add_edge(node, id);
    }
    for node in self.propagate_nodes[thread_id].clone() {
      self.graph.add_edge(id, node);
    }
    self.propagate_nodes[thread_id].insert(id);
  }
}

impl ThreadSystem for TSOThreadSystem {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.graph.execution_candidates.iter().map(|id| self.graph.instructions[*id].clone()).collect()
    }

    fn assign_register(&mut self, thread_id: usize, register: String, value: i32) {
      self.registers[thread_id].insert(register, value);
    }

    fn get_register(&self, thread_id: usize, register: String) -> i32 {
      match self.registers[thread_id].get(&register) {
        Some(value) => *value,
        None => 0
      }
    }

    fn remove_node(&mut self, node: &Node) {
      match node.instruction.instruction {
        instruction::Instruction::Propagate { thread_id: _, address: _, value: _ } => {
          self.propagate_nodes[node.thread_id].remove(&node.id);
        }
        _ => {}
      }
      self.graph.remove_node(node.id);
    }

    fn goto(&mut self, label: String) {
      if !self.graph.is_label_active(label.clone()) {
        let mut current_label: Option<String> = None;
        while current_label != Some(label.clone()) {
          current_label = self.graph.restore_node().clone();
        }
      }
    }
}


pub struct PSOThreadSystem {
  graph: Graph,
  registers: Vec<HashMap<String, i32>>,
  propagate_nodes: Vec<HashSet<(usize, i32)>>
}

impl Debug for PSOThreadSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# REGISTERS\n")?;
    for (i, register) in self.registers.iter().enumerate() {
      write!(f, "| Thread {}: {:?}\n", i, register)?;
    }
    Ok(())
  }
}

impl PSOThreadSystem {
  pub fn new(instructions: Vec<Vec<LabeledInstruction>>) -> PSOThreadSystem {
    let mut graph = Graph::new();
    let mut registers = Vec::new();
    let mut propagate_nodes = Vec::new();
    for _ in 0..instructions.len() {
      registers.push(HashMap::new());
      propagate_nodes.push(HashSet::new());
    }
    for (thread_id, thread_instructions) in instructions.iter().enumerate() {
      let mut instruction_ids: Vec<usize> = Vec::new();
      for instruction in thread_instructions.iter() {
        let id = graph.add_node(thread_id as usize, instruction.clone());
        instruction_ids.push(id);
      }
      for (i, instruction) in thread_instructions.iter().enumerate() {
        match instruction.get_mode() {
          Some(instruction::Mode::Rel) => {
            for j in i + 1..thread_instructions.len() {
              graph.add_edge(instruction_ids[j], instruction_ids[i]);
            }
          }
          Some(instruction::Mode::Acq) => {
            for j in 0..i {
              graph.add_edge(instruction_ids[i], instruction_ids[j]);
            }
          }
          Some(instruction::Mode::RelAcq) => {
            for j in 0..i {
              graph.add_edge(instruction_ids[i], instruction_ids[j]);
            }
            for j in i + 1..thread_instructions.len() {
              graph.add_edge(instruction_ids[j], instruction_ids[i]);
            }
          }
          Some(instruction::Mode::SeqCst) => {}
          Some(instruction::Mode::Rlx) => {}
          None => {}
        }
      }
    }
    PSOThreadSystem {
      graph,
      registers,
      propagate_nodes
    }
  }

  pub fn add_propagate_node(&mut self, thread_id: usize, address: i32, value: i32) {
    let id = self.graph.add_node(thread_id, LabeledInstruction {
      label: None,
      instruction: instruction::Instruction::Propagate { thread_id, address, value }
    });
    let active_fence_nodes = self.graph.active_fence_nodes.clone();
    for node in active_fence_nodes {
      self.graph.add_edge(node, id);
    }
    for (node, add) in self.propagate_nodes[thread_id].clone() {
      if address == add {
        self.graph.add_edge(id, node);
      }
    }
    self.propagate_nodes[thread_id].insert((id, address));
  }
}

impl ThreadSystem for PSOThreadSystem {
    fn get_possible_executions(&self) -> Vec<Node> {
      self.graph.execution_candidates.iter().map(|id| self.graph.instructions[*id].clone()).collect()
    }

    fn assign_register(&mut self, thread_id: usize, register: String, value: i32) {
      self.registers[thread_id].insert(register, value);
    }

    fn get_register(&self, thread_id: usize, register: String) -> i32 {
      match self.registers[thread_id].get(&register) {
        Some(value) => *value,
        None => 0
      }
    }

    fn remove_node(&mut self, node: &Node) {
      match node.instruction.instruction {
        instruction::Instruction::Propagate { thread_id: _, address, value: _ } => {
          self.propagate_nodes[node.thread_id].remove(&(node.id, address));
        }
        _ => {}
      }
      self.graph.remove_node(node.id);
    }

    fn goto(&mut self, label: String) {
      if !self.graph.is_label_active(label.clone()) {
        let mut current_label: Option<String> = None;
        while current_label != Some(label.clone()) {
          current_label = self.graph.restore_node().clone();
        }
      }
    }
}
