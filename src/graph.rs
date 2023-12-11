use std::collections::{HashSet, HashMap};

use crate::instruction::LabeledInstruction;

#[derive(Clone)]
pub struct Node {
  pub id: usize,
  pub thread_id: usize,
  pub instruction: LabeledInstruction
}

impl Node {
  pub fn new(id: usize, thread_id: usize,  instruction: LabeledInstruction) -> Node {
    Node {
      id,
      thread_id,
      instruction
    }
  }
}

pub struct Graph {
  label_to_node: HashMap<String, usize>,
  pub instructions: Vec<Node>,
  pub rev_edges: Vec<Vec<usize>>,
  pub active_neighbors: Vec<usize>,
  pub is_active: Vec<bool>,
  pub active_fence_nodes: HashSet<usize>,
  pub execution_stack: Vec<usize>,
  pub execution_candidates: HashSet<usize>
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      label_to_node: HashMap::new(),
      instructions: Vec::new(),
      rev_edges: Vec::new(),
      active_neighbors: Vec::new(),
      is_active: Vec::new(),
      active_fence_nodes: HashSet::new(),
      execution_stack: Vec::new(),
      execution_candidates: HashSet::new()
    }
  }

  pub fn is_label_active(&self, label: String) -> bool {
    match self.label_to_node.get(&label) {
      Some(id) => {
        self.is_node_active(*id)
      },
      None => {
        true
      }
    }
  }

  pub fn is_node_active(&self, id: usize) -> bool {
    self.is_active[id]
  }

  pub fn add_node(&mut self, thread_id: usize, instruction: LabeledInstruction) -> usize {
    let id = self.instructions.len();
    if instruction.label.is_some() {
      self.label_to_node.insert(instruction.label.clone().unwrap(), id);
    }
    if instruction.is_fence() {
      self.active_fence_nodes.insert(id);
    }
    self.instructions.push(Node::new(id, thread_id, instruction));
    self.rev_edges.push(Vec::new());
    self.active_neighbors.push(0);
    self.is_active.push(true);
    self.execution_candidates.insert(id);
    id
  }

  pub fn add_edge(&mut self, from: usize, to: usize) {
    if self.is_active[to] {
      self.active_neighbors[from] += 1;
    }
    self.rev_edges[to].push(from);
    if self.execution_candidates.contains(&from) {
      self.execution_candidates.remove(&from);
    }
  }

  pub fn remove_node(&mut self, id: usize) {
    if !self.is_active[id] {
      return;
    }
    if self.active_fence_nodes.contains(&id) {
      self.active_fence_nodes.remove(&id);
    }
    self.execution_stack.push(id);
    self.is_active[id] = false;
    self.execution_candidates.remove(&id);
    for from in self.rev_edges[id].iter() {
      if self.is_active[*from] {
        self.active_neighbors[*from] -= 1;
        if self.active_neighbors[*from] == 0 {
          self.execution_candidates.insert(*from);
        }
      }
    }
  }

  pub fn restore_node(&mut self) -> Option<String> {
    let id = self.execution_stack.pop().unwrap();
    self.is_active[id] = true;
    if self.instructions[id].instruction.is_fence() {
      self.active_fence_nodes.insert(id);
    }
    for from in self.rev_edges[id].iter() {
      if self.is_active[*from] {
        self.active_neighbors[*from] += 1;
        if self.active_neighbors[*from] == 1 {
          self.execution_candidates.remove(&from);
        }
      }
    }
    self.execution_candidates.insert(id);
    self.instructions[id].instruction.label.clone()
  }
}
