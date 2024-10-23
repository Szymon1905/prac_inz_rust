use std::f32::INFINITY;

pub fn play(name:String) {
    println!("Playing movie {}",name);
}

#[derive(Debug)]
pub struct Solution {
    pub(crate) cities: Vec<i32>,
    pub(crate) path_length: i32,
}

impl Solution {
    pub fn new(cities: Vec<i32>, path_length: i32) -> Self {
        Solution { cities, path_length }
    }
    pub fn reset(&mut self) {
        self.cities.clear();
        self.path_length = i32::MAX;
    }
}