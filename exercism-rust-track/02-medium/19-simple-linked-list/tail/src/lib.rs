use std::iter::FromIterator;

struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    length: usize,
    tail: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            length: 0,
            tail: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tail.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, _element: T) {
        self.length += 1;

        self.tail = Some(Box::new(Node {
            data: _element,
            prev: self.tail.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.length -= 1;

        let tail_node = self.tail.take().unwrap();
        self.tail = tail_node.prev;
        Some(tail_node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        self.tail.as_ref().map(|x| &x.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        if self.is_empty() {
            self
        } else {
            let mut result: SimpleLinkedList<T> = SimpleLinkedList::new();
            let mut tail = self.tail;

            while tail.is_some() {
                let tail_node = tail.take().unwrap();

                result.push(tail_node.data);

                tail = tail_node.prev;
            }

            result
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = SimpleLinkedList::new();

        for i in _iter {
            result.push(i);
        }

        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        if self.is_empty() {
            vec![]
        } else {
            let mut result = Vec::with_capacity(self.len());

            while !self.is_empty() {
                result.push(self.pop().unwrap());
            }

            result.reverse();

            result
        }
    }
}
