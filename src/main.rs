use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct MathSet<T> {
    elements: HashSet<T>,
}

impl<T> MathSet<T> where T: Eq + Hash + Clone, {
    fn new() -> Self{
        Self{
            elements: HashSet::new(),
        }
    }

    fn add(&mut self, value: T) {
        self.elements.insert(value);
    }

    fn remove(&mut self, value: &T) -> bool{
        self.elements.remove(value)
    }

    fn contains(&self, value: &T) -> bool{
        self.elements.contains(value)
    }

    fn union(&self, other: &MathSet<T>) -> MathSet<T> {
        let mut result = self.clone();
        for element in &other.elements {
            result.add(element.clone());
        }
        result
    }

    fn intersection(&self, other: &MathSet<T>) -> MathSet<T> {
        let mut result = MathSet::new();
        for item in &self.elements {
            if other.contains(item) {
                result.add(item.clone());
            }
        }
        result
    }

    fn difference(&self, other: &MathSet<T>) -> MathSet<T> {
        let mut result = MathSet::new();
        for item in &self.elements {
            if !other.contains(item) {
                result.add(item.clone());
            }
        }
        result
    }

    
    fn elements(&self) -> Vec<T> {
        self.elements.iter().cloned().collect()
    }
}

fn main(){
    let mut set1 = MathSet::new();
    set1.add(1);
    set1.add(2);
    set1.add(3);

    let mut set2 = MathSet::new();
    set2.add(3);
    set2.add(4);
    set2.add(5);


    println!("Set1: {:?}", set1.elements());
    println!("Set2: {:?}", set2.elements());

    let union = set1.union(&set2);
    println!("Union: {:?}", union.elements());

    let intersection = set1.intersection(&set2);
    println!("Intersection: {:?}", intersection.elements());

    let difference = set1.difference(&set2);
    println!("Difference: {:?}", difference.elements());

    let remove = set1.remove(&3);
    println!("Remove: {:?}", remove);

}