use std::collections::HashMap;
use core::fmt::Debug;

pub trait StorageSystem {
  fn load(&self, thread_id: usize, address: i32) -> i32;
  fn store(&mut self, thread_id: usize, address: i32, value: i32);
  fn cas(&mut self, thread_id: usize, address: i32, exp: i32, des: i32) -> i32;
  fn fai(&mut self, thread_id: usize, address: i32, inc: i32) -> i32;
}

pub struct SCStorageSystem {
  memory: HashMap<i32, i32>
}

impl SCStorageSystem {
  pub fn new() -> SCStorageSystem {
    SCStorageSystem {
      memory: HashMap::new()
    }
  }
}

impl Debug for SCStorageSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# MEMORY\n")?;
    write!(f, "| {:?}\n", self.memory)
  }
}

impl StorageSystem for SCStorageSystem {
  fn load(&self, _thread_id: usize, address: i32) -> i32 {
    match self.memory.get(&address) {
      Some(value) => *value,
      None => 0
    }
  }

  fn store(&mut self, _thread_id: usize, address: i32, value: i32) {
    self.memory.insert(address, value);
  }

  fn cas(&mut self, thread_id: usize, address: i32, exp: i32, des: i32) -> i32 {
    let value = self.load(thread_id, address);
    if value == exp {
      self.store(thread_id, address, des);
    }
    value
  }

  fn fai(&mut self, thread_id: usize, address: i32, inc: i32) -> i32 {
    let value = self.load(thread_id, address);
    self.store(thread_id, address, value + inc);
    value
  }
}

pub struct TSOStorageSystem {
  buffers: Vec<Vec<(i32, i32)>>,
  memory: HashMap<i32, i32>
}

impl Debug for TSOStorageSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# BUFFERS\n")?;
    for (i, buffer) in self.buffers.iter().enumerate() {
      write!(f, "| Thread {}: {:?}\n", i, buffer)?;
    }
    write!(f, "# MEMORY\n")?;
    write!(f, "| {:?}\n", self.memory)
  }
}

impl TSOStorageSystem {
  pub fn new(number_of_threads: usize) -> TSOStorageSystem {
    let mut buffers = Vec::new();
    for _ in 0..number_of_threads {
      buffers.push(Vec::new());
    }
    TSOStorageSystem {
      buffers,
      memory: HashMap::new()
    }
  }

  pub fn propagate(&mut self, thread_id: usize, address: i32) {
    let buffers_copy = self.buffers[thread_id].clone();
    let element = buffers_copy.iter().enumerate().rev().find(|(_, (a, _))| *a == address);
    match element {
      Some((i, (_, value) )) => {
        self.buffers[thread_id as usize].remove(i);
        self.memory.insert(address, *value);
      }
      _ => {}
    }
  }
}

impl StorageSystem for TSOStorageSystem {
  fn load(&self, thread_id: usize, address: i32) -> i32 {
    match self.buffers[thread_id as usize].iter().rev().find(|(a, _)| *a == address) {
      Some((_, value)) => *value,
      None => match self.memory.get(&address) {
        Some(value) => *value,
        None => 0
      }
    }
  }

  fn store(&mut self, thread_id: usize, address: i32, value: i32) {
    self.buffers[thread_id as usize].push((address, value));
  }

  fn cas(&mut self, thread_id: usize, address: i32, exp: i32, des: i32) -> i32 {
    let value = self.load(thread_id, address);
    if value == exp {
      self.store(thread_id, address, des);
    }
    value
  }

  fn fai(&mut self, thread_id: usize, address: i32, inc: i32) -> i32 {
    let value = self.load(thread_id, address);
    self.store(thread_id, address, value + inc);
    value
  }
}

pub struct PSOStorageSystem {
  buffers: Vec<Vec<(i32, i32)>>,
  memory: HashMap<i32, i32>
}

impl Debug for PSOStorageSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "# BUFFERS\n")?;
    for (i, buffer) in self.buffers.iter().enumerate() {
      write!(f, "| Thread {}: {:?}\n", i, buffer)?;
    }
    write!(f, "# MEMORY\n")?;
    write!(f, "| {:?}\n", self.memory)
  }
}

impl PSOStorageSystem {
  pub fn new(number_of_threads: usize) -> PSOStorageSystem {
    let mut buffers = Vec::new();
    for _ in 0..number_of_threads {
      buffers.push(Vec::new());
    }
    PSOStorageSystem {
      buffers,
      memory: HashMap::new()
    }
  }

  pub fn propagate(&mut self, thread_id: usize, address: i32) {
    let buffers_copy = self.buffers[thread_id].clone();
    let element = buffers_copy.iter().enumerate().rev().find(|(_, (a, _))| *a == address);
    match element {
      Some((i, (_, value) )) => {
        self.buffers[thread_id as usize].remove(i);
        self.memory.insert(address, *value);
      }
      _ => {}
    }
  }
}

impl StorageSystem for PSOStorageSystem {
  fn load(&self, thread_id: usize, address: i32) -> i32 {
    match self.buffers[thread_id as usize].iter().rev().find(|(a, _)| *a == address) {
      Some((_, value)) => *value,
      None => match self.memory.get(&address) {
        Some(value) => *value,
        None => 0
      }
    }
  }

  fn store(&mut self, thread_id: usize, address: i32, value: i32) {
    self.buffers[thread_id as usize].push((address, value));
  }

  fn cas(&mut self, thread_id: usize, address: i32, exp: i32, des: i32) -> i32 {
    let value = self.load(thread_id, address);
    if value == exp {
      self.store(thread_id, address, des);
    }
    value
  }

  fn fai(&mut self, thread_id: usize, address: i32, inc: i32) -> i32 {
    let value = self.load(thread_id, address);
    self.store(thread_id, address, value + inc);
    value
  }
}