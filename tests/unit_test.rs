use ariprog::{
  get_common_difference,
  get_nth_term,
  get_first_term,
  insert_arithmetic_means
};

#[test]
fn it_gets_common_difference() {
  assert_eq!(get_common_difference(15.0, 10.0), 5.0);
  assert_eq!(get_common_difference(10.0, 8.0), 3.0);
}

#[test]
fn it_gets_nth_term() {
  assert_eq!(get_nth_term(1.0, 2.0, 20.0), 39.0);
  assert_eq!(get_nth_term(0.0, 2.0, 103.0), 204.0);
}

#[test]
fn it_gets_first_term() {
  assert_eq!(get_first_term(39.0, 2.0, 20.0), 1.0);
  assert_eq!(get_first_term(204.0, 2.0, 103.0), 0.0);
}

#[test]
fn it_insert_arithmetic_means() {
  assert_eq!(
    insert_arithmetic_means(6, 14.0, 98.0),
    vec![14.0, 26.0, 38.0, 50.0, 62.0, 74.0, 86.0, 98.0]
  );
  assert_eq!(
    insert_arithmetic_means(3, 1.0, 13.0),
    vec![1.0, 4.0, 7.0, 10.0, 13.0]
  )
}
