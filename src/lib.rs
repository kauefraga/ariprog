/// Returns the common difference between two terms of an arithmetic progression
///
/// # Arguments
///
/// * `term` - Any term of the AP
/// * `previous_term` - The term before `term`
///
/// # Examples
///
/// ```
/// use ariprog;
/// let common_diff = ariprog::get_common_difference(10.0, 5.0); // Should return 5
/// let cd = ariprog::get_common_difference(12.0, 2.0); // should return 10
/// ```
pub fn get_common_difference(term: f32, previous_term: f32) -> f32 {
    term - previous_term
}

/// Returns the nth term of an arithmetic progression
///
/// # Arguments
///
/// * `first_term` - The first term of the AP
/// * `common_difference` - The common difference of the AP
/// * `nth_term_position` - The nth term position (nth) of the AP (e.g.: the twentieth term is in position 20)
///
/// # Examples
///
/// ```
/// use ariprog;
/// let nth_term = ariprog::get_nth_term(1.0, 2.0, 20.0); // Should return 39
/// ```
pub fn get_nth_term(first_term: f32, common_difference: f32, nth_term_position: f32) -> f32 {
    let nth_term = first_term + (nth_term_position - 1.0) * common_difference;

    nth_term
}

/// Returns the first term of an arithmetic progression
///
/// # Arguments
///
/// * `nth_term` - The nth term of the AP
/// * `common_difference` - The common difference of the AP
/// * `nth_term_position` - The nth term position (nth) of the AP (e.g.: the twentieth term is in position 20)
///
/// # Examples
///
/// ```
/// use ariprog;
/// let first_term = ariprog::get_first_term(-103.0, -2.0, 50.0); // Should return -5
/// ```
pub fn get_first_term(nth_term: f32, common_difference: f32, nth_term_position: f32) -> f32 {
    // an = a + (n - 1)d --> a = an + [(n - 1) * d * -1]
    let first_term = nth_term + ((nth_term_position - 1.0) * common_difference * -1.0);

    first_term
}

/// Returns an arithmetic progression with the first term + arithmetic means + nth_term, as a vector
///
/// # Arguments
///
/// * `how_many_in_between` - How many terms go between the first term and the nth term
/// * `first_term` - The first term of the AP
/// * `nth_term` - The nth term of the AP
///
/// # Examples
///
/// ```
/// use ariprog;
/// let ap = ariprog::insert_arithmetic_means(3, 1.0, 13.0); // Should return [1.0, 4.0, 7.0, 10.0, 13.0]
/// ```
pub fn insert_arithmetic_means(how_many_in_between: i32, first_term: f32, nth_term: f32) -> Vec<f32> {
    let how_many_terms = how_many_in_between as f32 + 2.0;
    let first_calc = nth_term - first_term;

    let common_difference = first_calc / (how_many_terms - 1.0);

    let mut counter = 0.0;
    let mut arithmetic_means = vec![];

    while counter < how_many_terms {
        arithmetic_means.push(first_term + (counter * common_difference));
        counter += 1.0;
    }

    arithmetic_means
}
