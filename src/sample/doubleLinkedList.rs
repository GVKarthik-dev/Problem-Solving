// struct DoubleLinkedList{
//     previous: Option<Box<DoubleLinkedList>>,
//     val: i32,
//     next: Option<Box<DoubleLinkedList>>,
// }

// impl DoubleLinkedList {
//     fn new(value:i32) -> Self {
//         DoubleLinkedList(
//             previous: None,
//             val: value,
//             next: None
//         )
//     }

//     fn append(&mut self, value: i32) -> Self {
//         let mut current = self;

//         while let Some(ref mut next_node) = current.next {
//             current = next_node;
//         }

//         let mut new_node = Box::new(DoubleLinkedList::new(value));
//         new_node.previous = Some(Box::new(DoubleLinkedList::new(current.val)));
//         current.next = Some(new_node);
//     }

//     fn clone(&self) -> DoubleLinkedList{
//         DoubleLinkedList{
//             previous: self.previous.clone(),
//             val: self.val,
//             next: self.next.clone()
//         }
//     }
// }