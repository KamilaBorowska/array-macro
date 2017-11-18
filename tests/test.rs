#[macro_use]
extern crate array_macro;

use std::panic::catch_unwind;

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

#[test]
fn const_expr() {
    const TWO: usize = 2;
    assert_eq!(array![|i| i; 2 + TWO], [0, 1, 2, 3]);
}

static mut CALLED_DROP: bool = false;

#[test]
fn panic_safety() {
    struct DontDrop;
    impl Drop for DontDrop {
        fn drop(&mut self) {
            unsafe {
                CALLED_DROP = true;
            }
        }
    }
    fn panicky() -> DontDrop {
        panic!();
    }
    assert!(catch_unwind(|| array![panicky(); 2]).is_err());
    assert_eq!(unsafe { CALLED_DROP }, false);
}

static mut DROP_COUNT: usize = 0;

#[test]
fn panic_safety_part_two() {
    struct DropOnlyThrice;
    impl Drop for DropOnlyThrice {
        fn drop(&mut self) {
            unsafe {
                DROP_COUNT += 1;
            }
        }
    }
    fn panicky(i: usize) -> DropOnlyThrice {
        if i == 3 {
            panic!();
        }
        DropOnlyThrice
    }
    assert!(catch_unwind(|| array![|i| panicky(i); 555]).is_err());
    assert_eq!(unsafe { DROP_COUNT }, 3);
}