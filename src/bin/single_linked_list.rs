#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct List<T: std::fmt::Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            data: elem,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                Some(head.data)
            }
        }
    }

    fn traverse(&self) {
        let mut traverse_head = &self.head;
        while let Some(n) = traverse_head {
            println!("{:?}", n);
            traverse_head = &n.next;
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.traverse();
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    list.push(4);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}
