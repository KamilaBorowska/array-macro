#[macro_use]
extern crate array_macro;

#[test]
fn simple_array() {
    assert_eq!(array![3; 5], [3, 3, 3, 3, 3]);
}

#[test]
fn callback_array() {
    assert_eq!(array![|x| x * 2; 3], [0, 2, 4]);
}

#[test]
fn outer_scope() {
    let x = 1;
    assert_eq!(array![x; 3], [1, 1, 1]);
}

#[test]
fn mutability() {
    let mut x = 1;
    assert_eq!(
        array![|_| {
            x += 1;
            x
        }; 3],
        [2, 3, 4]
    );
}

#[test]
fn big_array() {
    assert_eq!(&array!["x"; 333] as &[_], &["x"; 333] as &[_]);
}

#[test]
fn macro_within_macro() {
    assert_eq!(
        array![|x| array![|y| (x, y); 2]; 3],
        [[(0, 0), (0, 1)], [(1, 0), (1, 1)], [(2, 0), (2, 1)]]
    );
}
