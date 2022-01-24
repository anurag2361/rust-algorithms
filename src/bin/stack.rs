#[derive(Debug, Clone, PartialEq)]
struct StackNode<T: std::marker::Copy> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

#[derive(Debug, Clone)]
struct Stack<T: std::marker::Copy> {
    first: Option<Box<StackNode<T>>>,
    last: Option<Box<StackNode<T>>>,
    size: usize,
}

impl<T: std::marker::Copy + std::fmt::Debug + std::cmp::PartialEq> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            first: None,
            last: None,
            size: 0,
        }
    }

    fn push(&mut self, elem: T) -> usize {
        if self.first.is_none() {
            self.first = Some(Box::new(StackNode {
                value: elem,
                next: None,
            }));
            self.last = Some(Box::new(StackNode {
                value: elem,
                next: None,
            }));
        } else {
            let new_node = Some(Box::new(StackNode {
                value: elem,
                next: self.first.take(),
            }));
            self.first = new_node;
        }
        self.size += 1;
        self.size
    }

    fn traverse(&self) {
        let mut traverse_head = &self.first;
        while let Some(n) = traverse_head {
            println!("{:?}", n);
            traverse_head = &n.next;
        }
    }

    fn pop(&mut self) -> Option<Box<StackNode<T>>> {
        match self.first.take() {
            None => None,
            Some(mut first) => {
                self.first = first.next.take();
                Some(first)
            }
        }
    }
}

fn main() {
    let mut value: Stack<i32> = Stack::new();
    value.push(10);
    value.push(20);
    value.traverse();
    value.pop();
    value.traverse();
}
