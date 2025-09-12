use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;

/// Validates that the input is a valid permutation.
///
/// A valid permutation must satisfy all the following:
/// - All elements must be strictly positive integers.
/// - No element can exceed the length of the list (i.e., must be in the range 1 to n).
/// - All elements must be unique (no duplicates).
///
/// # Arguments
/// - `image`: A vector of integers representing the image of a permutation.
///
/// # Raises
/// - `ValueError`: If any of the above conditions are violated.
///
/// # Example (in Python)
/// ```python
/// validate_permutation([3, 1, 2])  # OK
/// validate_permutation([1, 2, 2])  # Raises ValueError (duplicate)
/// validate_permutation([0, 2, 3])  # Raises ValueError (not strictly positive)
/// ```
#[pyfunction]
pub fn validate_permutation(image: Vec<i32>) -> PyResult<()> {
    let mut seen = HashSet::new();
    let n = image.len();

    for &img in &image {
        if img < 1 {
            return Err(PyValueError::new_err(format!(
                "Expected all strictly positive values, but got {}.",
                img
            )));
        }
        if img as usize > n {
            return Err(PyValueError::new_err(format!(
                "The permutation is not injecting on its image. {} is out of bounds.",
                img
            )));
        }
        if !seen.insert(img) {
            return Err(PyValueError::new_err(format!(
                "It seems that the permutation is not bijective. {} appears multiple times.",
                img
            )));
        }
    }

    Ok(())
}

/// Validate that a tuple (Vec) of integers forms a valid cycle.
///
/// A cycle must contain only strictly positive integers.
///
/// Raises a Python `ValueError` if invalid.
#[pyfunction]
pub fn validate_cycle(cycle: Vec<i32>) -> PyResult<()> {
    for &element in &cycle {
        if element < 1 {
            return Err(PyValueError::new_err(format!(
                "Expected all strictly positive values, but got {}.",
                element
            )));
        }
    }
    Ok(())
}

/// Validates that a list of cycles form a proper cycle decomposition.
///
/// A valid decomposition must satisfy:
/// - All cycles are pairwise disjoint (no overlapping elements).
/// - All integers from 1 to the maximum value must appear exactly once.
///
/// # Arguments
/// - `cycles`: A list of positive integers vectors.
///
/// # Raises
/// - `ValueError`: If the decomposition is invalid.
#[pyfunction]
pub fn validate_cycle_decomposition(cycles: Vec<Vec<i32>>) -> PyResult<()> {
    // Check pairwise disjointness
    for i in 0..cycles.len() {
        for j in (i + 1)..cycles.len() {
            let set_a: HashSet<i32> = cycles[i].clone().into_iter().collect();
            let set_b: HashSet<i32> = cycles[j].clone().into_iter().collect();

            let intersection: HashSet<i32> = set_a.intersection(&set_b).copied().collect();
            if !intersection.is_empty() {
                return Err(PyValueError::new_err(format!(
                    "The cycles {:?} and {:?} don't have disjoint support.",
                    cycles[i], cycles[j]
                )));
            }
        }
    }

    let all_elements: HashSet<i32> = cycles
        .iter()
        .flat_map(|cycle| cycle.iter().copied())
        .collect();

    // Empty decomposition is technically valid
    if all_elements.is_empty() {
        return Ok(());
    }

    let max = *all_elements.iter().max().unwrap();
    let expected: HashSet<i32> = (1..=max).collect();

    if all_elements != expected {
        let missing: HashSet<i32> = expected.difference(&all_elements).copied().collect();
        return Err(PyValueError::new_err(format!(
            "Every element from 1 to the biggest permuted element must be included in some cycle,\n\
             but this is not the case for the element(s): {:?}",
            missing
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_permutation() {
        // valid permutations
        assert!(validate_permutation(vec![]).is_ok());
        assert!(validate_permutation(vec![1]).is_ok());
        assert!(validate_permutation(vec![3, 1, 2]).is_ok());
        // non valid permutations
        assert!(validate_permutation(vec![0]).is_err());
        assert!(validate_permutation(vec![0, 1, 2]).is_err());
        assert!(validate_permutation(vec![-1, 2, 3]).is_err());
        assert!(validate_permutation(vec![1, 2, 4]).is_err());
        assert!(validate_permutation(vec![1, 2, 2]).is_err());
    }

    #[test]
    fn test_validate_cycle() {
        // valid cycles
        assert!(validate_cycle(vec![]).is_ok());
        assert!(validate_cycle(vec![1, 2, 3]).is_ok());
        assert!(validate_cycle(vec![1, 2, 3, 4, 18, 29]).is_ok());
        // non valid cycles
        assert!(validate_cycle(vec![0]).is_err());
        assert!(validate_cycle(vec![0, 1, 2]).is_err());
        assert!(validate_cycle(vec![-1, 2, 3]).is_err());
    }

    #[test]
    fn test_validate_cycle_decomposition() {
        // valid cycle decompositions
        assert!(validate_cycle_decomposition(vec![]).is_ok());
        assert!(validate_cycle_decomposition(vec![vec![1]]).is_ok());
        assert!(validate_cycle_decomposition(vec![vec![1], vec![2], vec![3]]).is_ok());
        assert!(validate_cycle_decomposition(vec![vec![3, 2, 1], vec![4]]).is_ok());
        // non valid cycle decompositions
        assert!(validate_cycle_decomposition(vec![vec![3, 2, 1], vec![3]]).is_err());
        assert!(validate_cycle_decomposition(vec![vec![3, 2]]).is_err());
    }
}
