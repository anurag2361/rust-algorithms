// #[derive(Debug)]
// struct StackNode<T: std::marker::Copy> {
//     value: T,
//     next: Option<Box<StackNode<T>>>,
// }

// impl<T: std::marker::Copy> StackNode<T> {
//     fn new(value: T) -> StackNode<T> {
//         StackNode { value, next: None }
//     }
// }

// #[derive(Debug)]
// struct Stack<T: std::marker::Copy> {
//     first: Option<StackNode<T>>,
//     last: Option<StackNode<T>>,
//     size: usize,
// }

// impl<T: std::marker::Copy> Stack<T> {
//     fn new() -> Stack<T> {
//         Stack {
//             first: None,
//             last: None,
//             size: 0,
//         }
//     }

//     fn push(&mut self, elem: T) -> usize {
//         if self.first.is_none() {
//             self.first = Some(StackNode::new(elem));
//             self.last = Some(StackNode::new(elem));
//         } else {
//             let temp = &self.first;
//             self.first = Some(StackNode::new(elem));
//             self.first.as_ref().unwrap().next = Some(Box::new(Some(StackNode::new(elem)).unwrap()));
//         }
//         self.size += 1;
//         self.size
//     }
// }

fn main() {
    // let value = StackNode::new(10);
    // println!("{:?}", value)
}
