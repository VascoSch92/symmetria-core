use pyo3::prelude::*;

/// Converts a permutation to its integer representation.
///
/// Given a list of integers representing a permutation (e.g. `[3, 1, 2]`), this function
/// returns the number formed by concatenating its elements, in order.
#[pyfunction]
pub fn int_repr(image: Vec<u32>) -> u32 {
    let n = image.len();
    let mut integer_presentation = 0;

    for (i, &img) in image.iter().enumerate() {
        let power = n - i - 1;
        integer_presentation += img * 10_u32.pow(power as u32);
    }
    integer_presentation
}

/// Returns a string representation of the permutation in the form:
/// "Permutation(a, b, c, ...)".
#[pyfunction]
pub fn repr(image: Vec<u32>) -> String {
    let formatted: Vec<String> = image.iter().map(|v| v.to_string()).collect();
    format!("Permutation({})", formatted.join(", "))
}

/// Returns the 1-based ascent positions in the given permutation.
#[pyfunction]
pub fn ascents(image: Vec<u32>) -> Vec<u32> {
    image
        .windows(2)
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair[0] < pair[1] {
                Some((i + 1) as u32) // 1-based indexing
            } else {
                None
            }
        })
        .collect()
}

/// Returns the 1-based descent positions in the given permutation.
#[pyfunction]
pub fn descents(image: Vec<u32>) -> Vec<u32> {
    image
        .windows(2)
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair[0] > pair[1] {
                Some((i + 1) as u32) // 1-based indexing
            } else {
                None
            }
        })
        .collect()
}

/// Returns the positions where value is ≥ or > its 1-based index, based on `weakly`.
#[pyfunction]
pub fn exceedances(image: Vec<u32>, weakly: bool) -> Vec<u32> {
    image
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| {
            let pos = (i + 1) as u32; // 1-based index
            if (weakly && p >= pos) || (!weakly && p > pos) {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
}

/// Check if the image represents a derangement
#[pyfunction]
pub fn is_derangement(image: Vec<u32>) -> bool {
    !image
        .iter()
        .enumerate()
        .any(|(i, &val)| val == (i as u32 + 1))
}

/// Compute Lehmer code of a given permutation
#[pyfunction]
pub fn lehmer_code(image: Vec<u32>) -> Vec<u32> {
    let n = image.len();
    let mut lehmer_code = vec![0; n];
    let mut stack: Vec<(u32, u32)> = Vec::new(); // (value, count)

    for i in (1..=n).rev() {
        let mut count = 0;
        while let Some(&(val, old_count)) = stack.last() {
            if val < image[i - 1] {
                stack.pop();
                count += 1 + old_count;
            } else {
                break;
            }
        }
        lehmer_code[i - 1] = count;
        stack.push((image[i - 1], count));
    }

    lehmer_code
}

fn factorial(n: usize) -> u64 {
    (1..=n as u64).product()
}

/// Compute lexicographic rank of a permutation
#[pyfunction]
pub fn lexicographic_rank(image: Vec<u32>) -> u64 {
    let n = image.len();
    let mut rank: u64 = 1;

    for i in 0..n {
        let mut right_smaller: u64 = 0;
        for j in (i + 1)..n {
            if image[i] > image[j] {
                right_smaller += 1;
            }
        }
        rank += right_smaller * factorial(n - i - 1);
    }

    rank
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_int() {
        assert_eq!(int_repr(vec![1]), 1);
        assert_eq!(int_repr(vec![2, 1]), 21);
        assert_eq!(int_repr(vec![1, 2, 3]), 123);
        assert_eq!(int_repr(vec![3, 2, 1]), 321);
        assert_eq!(int_repr(vec![4, 3, 2, 1]), 4321);
        assert_eq!(int_repr(vec![]), 0);
    }

    #[test]
    fn test_permutation_repr() {
        assert_eq!(repr(vec![1]), "Permutation(1)");
        assert_eq!(repr(vec![1, 2]), "Permutation(1, 2)");
        assert_eq!(repr(vec![1, 3, 2]), "Permutation(1, 3, 2)")
    }

    #[test]
    fn test_permutation_ascents() {
        assert_eq!(ascents(vec![4, 3, 2, 1]), Vec::<u32>::new());
        assert_eq!(ascents(vec![1, 2, 3]), vec![1, 2]);
        assert_eq!(ascents(vec![3, 4, 6, 2, 1, 5]), vec![1, 2, 5]);
        assert_eq!(ascents(vec![3, 4, 5, 2, 1, 6, 7]), vec![1, 2, 5, 6]);
    }

    #[test]
    fn test_permutation_descents() {
        assert_eq!(descents(vec![1, 2, 3]), Vec::<u32>::new());
        assert_eq!(descents(vec![3, 4, 5, 2, 1, 6, 7]), vec![3, 4]);
        assert_eq!(descents(vec![4, 3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn test_permutation_exceedances() {
        assert_eq!(exceedances(vec![1, 2, 3], false), Vec::<u32>::new());
        assert_eq!(exceedances(vec![1, 2, 3], true), vec![1, 2, 3]);
        assert_eq!(exceedances(vec![4, 3, 2, 1], false), vec![1, 2]);
        assert_eq!(exceedances(vec![3, 4, 5, 2, 1, 6, 7], false), vec![1, 2, 3]);
        assert_eq!(
            exceedances(vec![3, 4, 5, 2, 1, 6, 7], true),
            vec![1, 2, 3, 6, 7]
        );
    }

    #[test]
    fn test_permutation_is_derangement() {
        assert_eq!(is_derangement(vec![1]), false);
        assert_eq!(is_derangement(vec![2, 1]), true);
        assert_eq!(is_derangement(vec![1, 3, 2]), false);
        assert_eq!(is_derangement(vec![1, 4, 3, 2]), false);
        assert_eq!(is_derangement(vec![1, 4, 5, 7, 3, 2, 6]), false);
        assert_eq!(is_derangement(vec![6, 4, 5, 7, 3, 2, 1]), true);
    }

    #[test]
    fn test_lehmer_code() {
        assert_eq!(lehmer_code(vec![1]), vec![0]);
        assert_eq!(lehmer_code(vec![2, 1]), vec![1, 0]);
        assert_eq!(lehmer_code(vec![2, 1, 3]), vec![1, 0, 0]);
        assert_eq!(lehmer_code(vec![1, 2, 3]), vec![0, 0, 0]);
        assert_eq!(lehmer_code(vec![1, 2, 3, 4]), vec![0, 0, 0, 0]);
        assert_eq!(lehmer_code(vec![2, 1, 3, 4]), vec![1, 0, 0, 0]);
        assert_eq!(lehmer_code(vec![4, 3, 2, 1]), vec![3, 2, 1, 0]);
        assert_eq!(lehmer_code(vec![4, 1, 3, 2]), vec![3, 0, 1, 0]);
        assert_eq!(
            lehmer_code(vec![4, 1, 3, 2, 7, 6, 5, 8]),
            vec![3, 0, 1, 0, 2, 1, 0, 0]
        );
    }

    #[test]
    fn test_lexicographic_rank() {
        assert_eq!(lexicographic_rank(vec![1]), 1);
        assert_eq!(lexicographic_rank(vec![1, 2]), 1);
        assert_eq!(lexicographic_rank(vec![2, 1]), 2);
        assert_eq!(lexicographic_rank(vec![1, 2, 3]), 1);
        assert_eq!(lexicographic_rank(vec![1, 3, 2]), 2);
        assert_eq!(lexicographic_rank(vec![3, 2, 1]), 6);
        assert_eq!(lexicographic_rank(vec![3, 2, 1, 4]), 15);
        assert_eq!(lexicographic_rank(vec![1, 2, 5, 4, 3]), 6);
    }
}
