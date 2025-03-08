#[derive(Debug)]
pub struct LinkedList{
    val: i32,
    next:Option<Box<LinkedList>>
}

impl LinkedList{
    pub fn new(value:i32)-> Self{
        LinkedList{
            val:value,next:None
        }
    }

    pub fn append(&mut self, value:i32) {
        match &mut self.next {
            Some(next_node) => next_node.append(value),
            // None => self.next = Some(Box::new(LinkedList::new(value))),

            None => self.next = Some(Box::new(LinkedList::new(value))), // ✅ Corrected

        }
    }
}

pub fn main() {
    let mut head:LinkedList = LinkedList::new(5);

    head.append(2);
    head.append(3);

    println!("{:?}", head);
}
