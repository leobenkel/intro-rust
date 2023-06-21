// LinkedList:
#[derive(Debug, PartialEq, Eq, Clone)]
enum LinkedList<T> {
    Empty,
    Elem { value: T, next: Box<LinkedList<T>> },
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList::Empty
    }

    /// To append value at the end of the list:
    fn append(&mut self, elem: T) {
        let mut current = self;
        loop {
            match current {
                LinkedList::Empty => {
                    *current = LinkedList::Elem {
                        value: elem,
                        next: Box::new(LinkedList::Empty),
                    };
                    break;
                }
                LinkedList::Elem { ref mut next, .. } => {
                    current = next;
                }
            }
        }
    }

    fn to_vec(&self) -> Vec<&T> {
        let mut v = Vec::new();
        let mut current = self;
        while let LinkedList::Elem {
            value: ref elem,
            ref next,
        } = *current
        {
            v.push(elem);
            current = next;
        }
        v
    }
}

mod test {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);

        assert_eq!(list.to_vec(), vec![&1,&2,&3,&4,&5]);
    }
}
