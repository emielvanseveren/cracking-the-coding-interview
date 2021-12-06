// Different states of a Node

#[derive(Clone)]
enum Node<T> {
    Empty,
    Tail { item: T },
    Link { item: T, next: Box<Node<T>> },
}

#[derive(Clone)]
struct Cursor<T> {
    curr: Node<T>,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self::Empty
    }

    pub fn push(&mut self, it: T) {
        match self {
            Self::Empty => self.to_tail(it),
            Self::Tail { .. } => self.to_link(it),
            Self::Link { next, .. } => next.push(it),
        };
    }

    fn to_link(&mut self, it: T) {
        *self = match self {
            Self::Tail { item } => Self::Link {
                item: *item,
                next: Box::new(Self::Tail { item: it }),
            },
            _ => panic!("something went wrong"),
        }
    }

    fn to_tail(&mut self, it: T) {
        *self = match self {
            Self::Empty => Self::Tail { item: it },
            Self::Link { item: _, next: _ } => Self::Tail { item: it },
            _ => panic!("Supplied value was not of correct type or variant"),
        }
    }
    fn to_next(&mut self, nxt: Node<T>) {
        *self = nxt;
    }
    fn to_empty(&mut self) {
        *self = std::mem::replace(self, Node::Empty);
    }
}

impl<T> Iterator for Cursor<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.curr {
            Node::Empty => None,
            Node::Tail { item } => {
                self.curr = Node::Empty;
                Some(item)
            }
            Node::Link { item, ref mut next } => {
                let mut n = Box::new(Node::Empty);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                Some(item)
            }
        }
    }
}

impl<T> IntoIterator for Node<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor { curr: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut linked_list = Node::new();
        linked_list.push(5);
        linked_list.push(3);
        linked_list.push(4);

        let mut iter = linked_list.into_iter();
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 4);
    }
}
