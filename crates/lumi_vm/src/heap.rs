use std::{collections::HashMap, ops::Deref};

use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HandleId(usize);

impl From<usize> for HandleId {
    fn from(value: usize) -> Self {
        HandleId(value)
    }
}

impl From<&usize> for HandleId {
    fn from(value: &usize) -> Self {
        HandleId(*value)
    }
}

impl Deref for HandleId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<usize> for HandleId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

// TODO: add heap management logic

/// Represents a heap entry in the Lumi VM.
#[derive(Debug, Clone)]
pub enum HeapEntry {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function(HandleId),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
}

/// The heap is a structure that holds dynamically allocated objects, arrays, and other data types.
#[derive(Debug, Clone)]
pub struct Heap {
    entries: Vec<HeapEntry>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            entries: Vec::new(),
        }
    }

    pub fn allocate(&mut self, entry: HeapEntry) -> HandleId {
        self.entries.push(entry);
        HandleId(self.entries.len())
    }

    pub fn get(&self, handle: HandleId) -> Option<&HeapEntry> {
        self.entries.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: HandleId) -> Option<&mut HeapEntry> {
        self.entries.get_mut(handle.0)
    }
}
