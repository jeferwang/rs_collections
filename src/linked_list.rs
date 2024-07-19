pub struct LinkedList<T> {
    pub(crate) head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub(crate) elem: T,
    pub(crate) next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, e: T) {
        let new_node = Box::new(Node {
            elem: e,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // self.head.as_ref().map(|node| { &node.elem })
        Some(&self.head.as_ref()?.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // self.head.as_mut().map(|node| { &mut node.elem })
        Some(&mut self.head.as_mut()?.elem)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}