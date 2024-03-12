use crate::{
  get_common_difference,
  get_nth_term,
  get_first_term
};

#[test]
fn test_get_common_difference() {
  assert_eq!(get_common_difference(15.0, 10.0), 5.0);
  assert_ne!(get_common_difference(10.0, 8.0), 3.0);
}

#[test]
fn test_get_nth_term() {
  assert_eq!(get_nth_term(1.0, 2.0, 20.0), 39.0);
  assert_ne!(get_nth_term(0.0, 2.0, 103.0), 200.0);
}

#[test]
fn test_get_first_term() {
  assert_eq!(get_first_term(39.0, 2.0, 20.0), 1.0);
  assert_ne!(get_first_term(200.0, 2.0, 103.0), 0.0);
}
