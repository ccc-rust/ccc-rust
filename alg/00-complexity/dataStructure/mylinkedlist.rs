// https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/linked_list.rs
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) -> () {
        let mut node = Box::new(Node::new(obj));
        unsafe {
            // Since we are adding node at the end, next will always be None
            node.next = None;
            node.prev = self.end;
            // Get a pointer to node
            let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.end {
                // This is the case of empty list
                None => self.start = node_ptr,
                Some(end_ptr) => (*end_ptr.as_ptr()).next = node_ptr,
            }

            self.end = node_ptr;
        }
        self.length = self.length + 1;
    }

    pub fn get<'a>(&'a mut self, index: i32) -> Option<&'a T> {
        return self.get_ith_node(self.start, index);
    }

    fn get_ith_node<'a>(&'a mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&'a T> {
        unsafe {
            match node {
                None => None,
                Some(next_ptr) => match index {
                    0 => Some(&(*next_ptr.as_ptr()).val),
                    _ => self.get_ith_node((*next_ptr.as_ptr()).next, index - 1),
                },
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            match self.start {
                Some(node) => write!(f, "{}", node.as_ref()),
                None => write!(f, ""),
            }
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            match self.next {
                Some(node) => write!(f, "{}, {}", self.val, node.as_ref()),
                None => write!(f, "{}", self.val),
            }
        }
    }
}


fn main() {
  let mut list = LinkedList::<i32>::new();
  list.add(7);
  list.add(5);
  list.add(9);
  println!("Linked List is {}", list);
  let item = list.get(1);
  println!("item={:?}", item);
}
