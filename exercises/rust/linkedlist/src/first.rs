use std::mem;


struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

pub struct LinkedStack {
    head: Option<Box<Node>>,
}


impl LinkedStack {
    pub fn new() -> LinkedStack {
        LinkedStack { head: None }
    }

    pub fn push(&mut self, v: i32) {
        let n = Node {
            elem: v,
            next: mem::replace(&mut self.head, None),
        };
        self.head = Some(Box::new(n));
    }

    pub fn peek(&mut self) -> Option<i32> {
        match &self.head {
            None => None,
            Some(n) => Some((*n).elem),
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let None = &self.head {
            return None
        }
        let next = mem::replace(&mut self.head, None);
        let node = next.unwrap();
        self.head = node.next;
        Some(node.elem)
    }
}

// Note:
// mem::replace 似乎可以将引用对象中的字段 move 给另一个变量；
// 遇到 annot move out of borrowed content 可以使用它来处理？
// https://github.com/rust-unofficial/patterns/blob/master/idioms/mem-replace.md


#[test]
fn test_list() {
    let mut l = LinkedStack::new();
    l.push(1);
    l.push(2);
    assert_eq!(l.peek(), Some(2));
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), Some(1));
}
