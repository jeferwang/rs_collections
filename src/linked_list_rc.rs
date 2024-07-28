use std::rc::Rc;

pub struct LinkedListRc<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedListRc<T> {
    pub fn new() -> Self {
        LinkedListRc { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        LinkedListRc { head: Some(Rc::new(Node { elem, next: self.head.clone() })) }
    }

    pub fn tail(&self) -> Self {
        LinkedListRc { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct LinkedListRcIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedListRc<T> {
    pub fn iter(&self) -> LinkedListRcIter<T> {
        LinkedListRcIter { next: self.head.as_deref() }
    }
}

impl<'a,T> Iterator for LinkedListRcIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
