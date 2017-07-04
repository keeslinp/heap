use errors::Errors;
use std::cmp;
#[derive(Debug)]
pub struct Heap {
    values: Vec<(i32, char)>,
}

impl Heap {
    fn new() -> Self {
        Heap {
           values: Vec::new(),
        }
    }

    fn peek(&self) -> Result<(i32, char), Errors> {
        if self.values.len() > 0 {
            Ok(self.values[0])
        } else {
            Err(Errors::EmptyHeap)
        }
    }

    fn push(&mut self, (freq, val): (i32, char)) {
        self.values.push((freq, val));
        let mut index = self.values.len() - 1;
        while index > 0 {
            let parent_index = (index - if index % 2 == 0 { 2 } else { 1 }) / 2;
            if index <= 0 || freq <= self.values[parent_index].0 {
                break;
            }
            self.values.swap(parent_index, index);
            index = parent_index;
        }
    }

    fn pop(&mut self) -> Result<(i32, char), Errors> {
        let len = self.values.len();
        if len > 0  {
            self.values.swap(0, len - 1);
            println!("{:?}", &self.values);
            let mut index = 0;
            while (index * 2 + 1) < len - 2 {
                let left_child = index * 2 + 1;
                let right_child = cmp::min(left_child + 1, len - 2);
                let bigger_child = if self.values[right_child].0 > self.values[left_child].0 {
                    right_child
                } else {
                    left_child
                };
                if self.values[bigger_child].0 > self.values[index].0 {
                    self.values.swap(bigger_child, index);
                    println!("{:?}", &self.values);
                } else {
                    break;
                }
                index = bigger_child;
            }
            Ok(self.values.pop().unwrap())
        } else {
            Err(Errors::EmptyHeap)
        }
    }
}

#[cfg(test)]
mod tests {
    use heap::*;
    #[test]
    fn it_works() {
    }

    #[test]
    fn error_on_empty() {
        let heap = Heap::new();
        assert!(heap.peek().is_err());
    }

    #[test]
    fn add_one() {
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        assert!(heap.values[0] == (10, 'a'));
    }

    #[test]
    fn add_bigger() {
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        heap.push((15, 'b'));
        assert!(heap.values[0] == (15, 'b'));
    }

    #[test]
    fn add_smaller() {
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        heap.push((5, 'b'));
        assert!(heap.values[0] == (10, 'a'));
    }

    #[test]
    fn pop() {
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        heap.push((5, 'b'));
        assert!(heap.pop().unwrap() == (10, 'a'));
    }

    #[test]
    fn pop_empty() {
        let mut heap = Heap::new();
        assert!(heap.pop().is_err());
    }

    #[test]
    fn pop_many() {
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        heap.push((8, 'b'));
        heap.push((6, 'c'));
        heap.push((11, 'd'));
        heap.push((9, 'e'));
        assert!(heap.pop().unwrap() == (11, 'd'));
        assert!(heap.pop().unwrap() == (10, 'a'));
    }

    #[test]
    fn peak(){
        let mut heap = Heap::new();
        heap.push((10, 'a'));
        assert!(heap.peek().unwrap() == (10, 'a'));
    }
}
