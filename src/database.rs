use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct DataBase {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    children: HashMap<String, DataBase>,
}

unsafe impl Send for DataBase {}
unsafe impl Sync for DataBase {}
