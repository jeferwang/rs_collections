use std::rc::Rc;

pub struct LinkedList<T> {
    pub(crate) head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub(crate) struct Node<T> {
    pub(crate) elem: T,
    pub(crate) next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        LinkedList { head: Some(Rc::new(Node { elem, next: self.head.clone() })) }
    }

    pub fn tail(&self) -> Self {
        LinkedList { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}