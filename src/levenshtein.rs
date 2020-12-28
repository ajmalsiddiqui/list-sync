use std::cmp::{max,min};

// This module will be private to anybody who uses mod levenshtein from outside, but it will be
// accessible for sibling modules and items
mod util {
    pub fn print_table(table: &Vec<Vec<usize>>) {
        for row in table {
            for item in row {
                print!("{} ", item);
            }
            println!("");
        }
    }

    // Returns an initialized distance table of dimensions m+1 * n+1
    // This function will transfer ownership of `distances` to calling function
    pub fn get_distance_table<T: Default + Clone, F: Fn(usize) -> T + Copy>(m: usize, n: usize, init: F)-> Vec<Vec<T>> {
        let mut distances: Vec<Vec<T>> = Vec::new();

        distances.push((0..n+1).map(init).collect());

        for i in 1..m+1 {
            // initialize the whole row to sentinel
            //  TODO change this to -1
            distances.push(vec![T::default(); n+1]);
            // update the first item in the row
            distances[i][0] = init(i);
        }

        distances
    }
}

use util::*;

pub fn levenshtein_naive<T>(i1: &[T], i2: &[T]) -> usize
    where T: Eq
{
    // indices
    let i: usize = i1.len();
    let j: usize = i2.len();

    // base case
    if min(i, j) == 0 {
        max(i, j)
    } else if i1[i-1] == i2[j-1] {
        levenshtein_naive(&i1[..i-1], &i2[..j-1])
    } else {

        let delete = levenshtein_naive(&i1[..i-1], &i2);
        let insert = levenshtein_naive(&i1, &i2[..j-1]);
        let substitute = levenshtein_naive(&i1[..i-1], &i2[..j-1]);

        // implicit returns
        1 + insert.min(delete).min(substitute)
    }
}

pub fn levenshtein_tabulation<T>(i1: &[T], i2: &[T]) -> usize
    where T: Eq
{
    let m = i1.len();
    let n = i2.len();

    // TODO learn about arrays in rust and see if I can get something where I can define the size
    // before hand to improve performance

    // table of distances
    let mut distances:Vec<Vec<usize>> = get_distance_table(m, n, |i| (i));

    for i in 1..m+1 {
        for j in 1..n+1 {
            // returns value from the if/else expression and assigns to k
            let k = if i1[i-1] == i2[j-1] {
                0
            } else {
                1
            };

            let delete = distances[i-1][j];
            let insert = distances[i][j-1];
            let substitute = distances[i-1][j-1];

            distances[i][j] = k + delete.min(insert).min(substitute);
        }
    }

    print_table(&distances);

    distances[m][n]
}

pub fn levenshtein_memoization<T>(i1: &[T], i2: &[T]) -> usize
    where T: Eq
{

    // This funciton actually does all the recursion
    // i and j are the indices of s1 and s2 being considered
    // distances is a mutable reference because obviously it'll be filled up as needed
    fn levenshtein_memoization_helper<T>(i1: &[T], i2: &[T], i: usize, j: usize, distances: &mut Vec<Vec<Option<usize>>>) -> usize
        where T: Eq
    {
        // check the cache first
        if distances[i][j].is_none() {

            // couldn't find the value, time to recursively calculate it and update the cache
            
            // returns value from the if/else expression and assigns to distances[i][j]
            distances[i][j] = {
                // base case
                if min(i, j) == 0 {
                    Some(max(i, j))
                } else if i1[i-1] == i2[j-1] {
                    Some(levenshtein_memoization_helper(i1, i2, i-1, j-1, distances))
                } else {
                    // note that we don't need to pass distances as &mut distances
                    // because distances is already a mutable reference (see func signature)
                    Some(
                        1 + levenshtein_memoization_helper(i1, i2, i-1, j, distances)
                        .min(levenshtein_memoization_helper(i1, i2, i, j-1, distances))
                        .min(levenshtein_memoization_helper(i1, i2, i-1, j-1, distances))
                    )
                }
            };
        }
            
        // this is returned implicitly
        distances[i][j].unwrap()

    }

    let m = i1.len();
    let n = i2.len();

    let mut distances: Vec<Vec<Option<usize>>> = get_distance_table(m, n, |i| (Some(i)));

    let distance = levenshtein_memoization_helper(i1, i2, m, n, &mut distances);

    // returned implicitly
    distance
}

#[cfg(test)]
mod tests {
    // TODO see if I can have a relative path here that doesn't use crate::
    use crate::levenshtein::*;

    #[test]
    fn levenshtein_test() { 
        let s1 = "SATURDAY";
        let s2 = "SUNDAY";
        let expected_leven = 3;

        // TODO fix to use more intuitive string types
        // This is terrible. It won't work with non-ASCII stuff. But without this terribleness I
        // don't know how I'll make it work with any generic indexable type that contains
        // comparable types
        // TODO consider using as_str maybe
        // Note: as_bytes() returns a byte slice so we don't need &s2.as_bytes()[..]
        let leven_naive = levenshtein_naive(&s1.as_bytes(), &s2.as_bytes());
        let leven_tab = levenshtein_tabulation(&s1.as_bytes(), &s2.as_bytes());
        let leven_memo = levenshtein_memoization(&s1.as_bytes(), &s2.as_bytes());

        assert_eq!(leven_naive, expected_leven);
        assert_eq!(leven_tab, expected_leven);
        assert_eq!(leven_memo, expected_leven);

        let expected_leven = 2;
        let s3 = String::from("LAWN");
        let s4 = String::from("FLAW");

        let leven_naive = levenshtein_naive(&s3.as_bytes(), &s4.as_bytes());
        let leven_tab = levenshtein_tabulation(&s3.as_bytes(), &s4.as_bytes());
        let leven_memo = levenshtein_memoization(&s3.as_bytes(), &s4.as_bytes());

        assert_eq!(leven_naive, expected_leven);
        assert_eq!(leven_tab, expected_leven);
        assert_eq!(leven_memo, expected_leven);
    }
}
