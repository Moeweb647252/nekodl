use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use tokio::task::JoinHandle;

pub struct Task<T> {
    pub name: String,
    pub id: usize,
    pub handle: JoinHandle<T>,
}

pub struct TaskPool {
    data: HashMap<TypeId, Vec<(usize, Box<dyn Any>)>>,
    index: usize,
}

impl TaskPool {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            index: 0,
        }
    }

    pub fn add_task<T: Any>(&mut self, name: String, handle: JoinHandle<T>) -> usize {
        self.index += 1;
        let task = Task {
            name,
            id: self.index,
            handle,
        };
        if let Some(v) = self.data.get_mut(&TypeId::of::<T>()) {
            v.push((self.index, Box::new(task)));
        } else {
            self.data
                .insert(TypeId::of::<T>(), vec![(self.index, Box::new(task))]);
        }
        self.index
    }

    pub fn get_task<T: Any>(&self, id: usize) -> Option<&Task<T>> {
        self.data.get(&TypeId::of::<T>()).map(|v| {
            v.iter()
                .find(|t| t.0 == id)
                .unwrap()
                .1
                .downcast_ref()
                .unwrap()
        })
    }

    pub fn pop_task<T: Any>(&mut self, id: usize) -> Option<Box<Task<T>>> {
        let task_vec = self.data.get_mut(&TypeId::of::<T>())?;
        let pos = task_vec.iter().position(|t| t.0 == id)?;
        task_vec.remove(pos).1.downcast().ok()
    }
}
