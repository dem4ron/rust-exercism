#[derive(Debug)]
pub struct CustomSet<T> {
    collection: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.collection.iter().all(|x| other.contains(&x)) &&
        other.collection.iter().all(|x| self.contains(&x))
    }
}

impl<T: Ord + Clone> CustomSet<T> {
    pub fn new(inputs: Vec<T>) -> CustomSet<T> {
        let mut s = CustomSet { collection: Vec::new() };
        for input in inputs {
            s.add(input);
        }
        s
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.collection.push(element)
        }
    }

    pub fn delete(&mut self, element: &T) {
        self.collection.retain(|x| x != element)
    }

    pub fn contains(&self, other: &T) -> bool {
        self.collection.contains(other)
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn size(&self) -> usize {
        self.collection.len()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.collection.iter().all(|x| other.contains(x))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.collection.iter().any(|x| other.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .cloned()
                           .filter(|c| other.contains(c))
                           .collect())
    }

    pub fn union(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .cloned()
                           .chain(other.collection.iter().cloned())
                           .collect())
    }

    pub fn difference(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .cloned()
                           .filter(|c| !other.contains(c))
                           .collect())
    }

    pub fn symmetric_difference(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.difference(other)
                           .collection
                           .iter()
                           .cloned()
                           .chain(other.difference(self).collection.iter().cloned())
                           .collect())
    }
}