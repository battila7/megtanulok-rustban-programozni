use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

fn has_two_subsequent_nodes<T>(node: &Node<T>) -> bool {
    node.next.as_ref().unwrap().next.is_some()
}

fn has_subsequent_node<T>(node: &Node<T>) -> bool {
    node.next.is_some()
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_len = 0;
        let mut current = &self.head;

        while current.is_some() {
            current_len += 1;
            current = &current.as_ref().unwrap().next;
        };

        current_len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: None,
        });

        if self.is_empty() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();

            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            };

            current.next = Some(new_node);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.has_only_one_node() {
            let last_node = self.head.take().unwrap();

            Some(last_node.data)
        } else {
            let mut current = self.head.as_mut().unwrap();

            while has_two_subsequent_nodes(current) {
                current = current.next.as_mut().unwrap();
            }
            
            let last_node = current.next.take().unwrap();

            Some(last_node.data)
        }
    }

    fn has_only_one_node(&self) -> bool {
        self.head.as_ref().unwrap().next.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let mut current = self.head.as_ref().unwrap();

            while current.next.is_some() {
                current = current.next.as_ref().unwrap();
            }

            Some(&current.data)
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        if self.is_empty() {
            self
        } else {
            let mut result: SimpleLinkedList<T> = SimpleLinkedList::new();

            Self::recursive_rev(self.head.as_mut().unwrap(), &mut result);
            result.push(self.head.unwrap().data);

            result
        }
    }

    fn recursive_rev(current: &mut Box<Node<T>>, result: &mut SimpleLinkedList<T>) {
        if has_subsequent_node(current) {
            Self::recursive_rev(current.next.as_mut().unwrap(), result);

            let last = current.next.take().unwrap();

            result.push(last.data);
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

            recursive_into_vec(self.head.as_mut().unwrap(), &mut result);
            result.push(self.head.unwrap().data);
            result.reverse();

            result
        }
    }
}

fn recursive_into_vec<T>(current: &mut Box<Node<T>>, result: &mut Vec<T>) {
    if has_subsequent_node(current) {
        recursive_into_vec(current.next.as_mut().unwrap(), result);

        let last = current.next.take().unwrap();

        result.push(last.data);
    }
}
