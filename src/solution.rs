use std::f32::INFINITY;


#[derive(Debug)]
#[derive(Clone)]
pub struct Solution {
    pub(crate) cities: Vec<i32>,
    pub(crate) path_length: i32,
}

//struktura danych Solution i jej metody reprezentującr pojedyńcze rozwiązanie
impl Solution {
    pub fn new(cities: Vec<i32>, path_length: i32) -> Self {
        Solution { cities, path_length }
    }
    pub fn reset(&mut self) {
        self.cities.clear();
        self.path_length = i32::MAX;
    }
}