#[derive(Debug, PartialEq)]
struct NewNode {
    val: i32,
    next: Option<Box<NewNode>>,
}

#[derive(Debug)]
struct Queue {
    first: Option<Box<NewNode>>,
    last: Option<Box<NewNode>>,
    size: usize,
}

impl Queue {
    fn new() -> Queue {
        Queue {
            first: None,
            last: None,
            size: 0,
        }
    }

    fn enqueue(&mut self, value: i32) -> usize {
        if self.first.is_none() && self.last.is_none() {
            self.first = Some(Box::new(NewNode {
                val: value,
                next: None,
            }));
            self.last = Some(Box::new(NewNode {
                val: value,
                next: None,
            }));
        } else {
            let new_node = NewNode {
                val: value,
                next: self.last.take(),
            };
            self.last = Some(Box::new(new_node));
        }
        self.size += 1;
        self.size
    }

    fn dequeue(&mut self) -> Option<Box<NewNode>> {
        match self.first.take() {
            None => None,
            Some(mut first) => {
                self.first = first.next.take();
                Some(first)
            }
        }
    }

    fn traverse(&mut self) {
        let mut traverse_head = &self.last;
        while let Some(n) = traverse_head {
            println!("{:?}", n);
            traverse_head = &n.next;
        }
    }
}

fn main() {
    let mut new_q = Queue::new();
    new_q.enqueue(10);
    new_q.enqueue(20);
    new_q.traverse();
    new_q.dequeue();
    new_q.traverse()
}
