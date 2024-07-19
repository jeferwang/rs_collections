use rs_collections::LinkedList;

#[test]
fn into_iter() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);

    let mut iter = list.iter_mut();
    iter.next().map(|val| *val += 1);
    iter.next().map(|val| *val += 1);
    iter.next().map(|val| *val += 1);
    iter.next().map(|val| *val += 1);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), None);
}