use rs_collections::linked_list_rc::LinkedList;

#[test]
fn basics() {
    let list = LinkedList::new();
    assert_eq!(list.head(), None);

    let list = list.prepend(1).prepend(2).prepend(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
}

#[test]
fn head() {
    let list = LinkedList::new();
    let list = list.prepend(666);
    assert_eq!(list.head(), Some(&666))
}

#[test]
fn long_list() {
    let mut list = LinkedList::new();
    for i in 0..100000 {
        list = list.prepend(i);
    }
    drop(list);
}

#[test]
fn iter() {
    let list = LinkedList::new().prepend(1).prepend(2).prepend(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}