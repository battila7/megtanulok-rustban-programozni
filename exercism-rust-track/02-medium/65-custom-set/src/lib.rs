use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

const PLACEHOLDER: u8 = 0;

#[derive(Debug)]
pub struct CustomSet<T>
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    data: HashMap<T, u8>,
}

impl<T> PartialEq for CustomSet<T>
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T> CustomSet<T>
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    pub fn new(_input: &[T]) -> Self {
        let mut data = HashMap::new();

        _input.iter().for_each(|e| {
            data.insert(e.clone(), PLACEHOLDER);
        });

        Self { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains_key(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.data.insert(_element, PLACEHOLDER);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.is_empty() {
            return true;
        }

        self.data.keys().all(|key| _other.contains(key))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.intersection(_other).is_empty()
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        let mut new_set = Self {
            data: HashMap::new(),
        };

        self.data
            .keys()
            .filter(|key| _other.contains(key))
            .for_each(|e| {
                new_set.data.insert(e.clone(), PLACEHOLDER);
            });

        new_set
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let mut new_set = Self {
            data: HashMap::new(),
        };

        self.data
            .keys()
            .filter(|key| !_other.contains(key))
            .for_each(|e| {
                new_set.data.insert(e.clone(), PLACEHOLDER);
            });

        new_set
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut new_set = Self {
            data: HashMap::new(),
        };

        self.data.keys().chain(_other.data.keys()).for_each(|e| {
            new_set.data.insert(e.clone(), PLACEHOLDER);
        });

        new_set
    }
}
