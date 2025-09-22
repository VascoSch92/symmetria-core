use pyo3::prelude::*;
use std::collections::HashMap;

/// Cycle acting on an integer
#[pyfunction]
pub fn call_on_int(cycle: Vec<u32>, idx: u32) -> u32 {
    if let Some(index) = cycle.iter().position(|&x| x == idx) {
        let next_index = (index + 1) % cycle.len();
        cycle[next_index]
    } else {
        idx
    }
}

/// Cycle acting on a string
#[pyfunction]
pub fn call_on_str(cycle: Vec<u32>, string: String) -> String {
    let mut permuted: Vec<char> = string.chars().collect();
    let original_chars: Vec<char> = string.chars().collect();

    for (i, &ch) in original_chars.iter().enumerate() {
        // 1-based index for permutation
        let target_pos = call_on_int(cycle.clone(), (i + 1) as u32) - 1;

        if let Ok(pos) = usize::try_from(target_pos) {
            if pos < permuted.len() {
                permuted[pos] = ch;
            }
        }
    }

    permuted.into_iter().collect()
}

/// Converts a cycle into its integer representation
#[pyfunction]
pub fn int_repr(cycle: Vec<u32>) -> u32 {
    let n = cycle.len();
    cycle
        .iter()
        .enumerate()
        .map(|(i, &img)| {
            let power = n - i - 1;
            img * 10_u32.pow(power as u32)
        })
        .sum()
}

/// Compute the inversions of a permutation
#[pyfunction]
pub fn inversions(cycle: Vec<u32>) -> Vec<(u32, u32)> {
    let mut inversions: Vec<(u32, u32)> = Vec::new();
    let mut min_element: u32 = 0;

    for (i, p) in cycle.clone().into_iter().enumerate() {
        if p == min_element {
            min_element += 1;
        } else {
            for (j, q) in cycle[i..].iter().enumerate().skip(1) {
                if p > *q {
                    inversions.push(((i + 1).try_into().unwrap(), (i + j + 1).try_into().unwrap()))
                }
            }
        }
    }

    inversions
}

/// Returns the map representing the cycle
#[pyfunction]
pub fn map(cycle: Vec<u32>) -> HashMap<u32, u32> {
    cycle
        .iter()
        .cloned()
        .zip(cycle.iter().cycle().skip(1).cloned())
        .collect()
}

/// Representation of a cycle
#[pyfunction]
pub fn repr(cycle: Vec<u32>) -> String {
    format!(
        "Cycle({})",
        cycle
            .iter()
            .map(|element| element.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

/// The method standardize a cycle.
/// The unique standardized forms of a cycle is the one
/// where the first element is the smallest
#[pyfunction]
pub fn standardization(cycle: Vec<u32>) -> Vec<u32> {
    let smallest_index = cycle
        .iter()
        .enumerate()
        .min_by_key(|(_, &value)| value)
        .map(|(index, _)| index)
        .unwrap_or(0);

    cycle
        .clone()
        .into_iter()
        .cycle()
        .skip(smallest_index)
        .take(cycle.len())
        .collect()
}

/// Stringed form of a cycle
#[pyfunction]
pub fn str(cycle: Vec<u32>) -> String {
    format!(
        "({})",
        cycle
            .iter()
            .map(|element| element.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_on_int() {
        assert_eq!(call_on_int(vec![2, 1], 2), 1);
        assert_eq!(call_on_int(vec![1, 2, 3], 1), 2);
        assert_eq!(call_on_int(vec![1, 2, 3], 2), 3);
        assert_eq!(call_on_int(vec![1, 2, 3], 3), 1);
        assert_eq!(call_on_int(vec![1, 3, 2], 17), 17);
        assert_eq!(call_on_int(vec![1, 2, 3], 10), 10);
        assert_eq!(call_on_int(vec![2, 1, 3], 1), 3);
        assert_eq!(call_on_int(vec![4, 5, 6, 3, 2, 1], 4), 5);
    }

    #[test]
    fn test_call_on_str() {
        assert_eq!(call_on_str(vec![1], "ab".to_string()), "ab".to_string());
        assert_eq!(call_on_str(vec![2, 1], "ab".to_string()), "ba".to_string());
        assert_eq!(
            call_on_str(vec![3, 1, 2], "abc".to_string()),
            "cab".to_string()
        );
    }

    #[test]
    fn test_int_repr() {
        assert_eq!(int_repr(vec![1]), 1);
        assert_eq!(int_repr(vec![1, 3, 4]), 134);
        assert_eq!(int_repr(vec![1, 3, 4, 5, 6, 7, 8, 9]), 13456789);
    }

    #[test]
    fn test_inversions() {
        assert_eq!(inversions(vec![1, 2, 3]), Vec::new());
        assert_eq!(inversions(vec![1, 3, 4, 2]), vec![(2, 4), (3, 4)]);
        assert_eq!(
            inversions(vec![3, 1, 2, 5, 4]),
            vec![(1, 2), (1, 3), (4, 5)]
        );
    }

    #[test]
    fn test_map() {
        let input = vec![1, 2, 3, 4];
        let result = map(input.clone());

        let expected: HashMap<u32, u32> =
            vec![(1, 2), (2, 3), (3, 4), (4, 1)].into_iter().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_repr() {
        assert_eq!(repr(vec![1]), "Cycle(1)".to_string());
        assert_eq!(repr(vec![1, 2, 3]), "Cycle(1 2 3)".to_string());
        assert_eq!(repr(vec![1, 2]), "Cycle(1 2)".to_string());
    }

    #[test]
    fn test_standardization() {
        assert_eq!(standardization(vec![1]), vec![1]);
        assert_eq!(standardization(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(standardization(vec![7, 3, 4]), vec![3, 4, 7]);
        assert_eq!(
            standardization(vec![10, 9, 8, 4, 5, 1]),
            vec![1, 10, 9, 8, 4, 5]
        );
    }

    #[test]
    fn test_str() {
        assert_eq!(str(vec![1]), "(1)".to_string());
        assert_eq!(str(vec![1, 2, 3]), "(1 2 3)".to_string());
        assert_eq!(str(vec![1, 2]), "(1 2)".to_string());
    }
}
