mod elements;

use crate::elements::permutation::{
    ascents, descents, exceedances, int_repr, is_derangement, lehmer_code, lexicographic_rank, repr,
};
use crate::elements::validators::{
    validate_cycle, validate_cycle_decomposition, validate_permutation,
};

use pyo3::prelude::*;

#[pymodule]
fn _symmetria_core(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let validators = PyModule::new(py, "validators")?;
    validators.add_function(wrap_pyfunction!(validate_permutation, py)?)?;
    validators.add_function(wrap_pyfunction!(validate_cycle, py)?)?;
    validators.add_function(wrap_pyfunction!(validate_cycle_decomposition, py)?)?;

    let permutation = PyModule::new(py, "permutation")?;
    permutation.add_function(wrap_pyfunction!(ascents, py)?)?;
    permutation.add_function(wrap_pyfunction!(descents, py)?)?;
    permutation.add_function(wrap_pyfunction!(exceedances, py)?)?;
    permutation.add_function(wrap_pyfunction!(int_repr, py)?)?;
    permutation.add_function(wrap_pyfunction!(is_derangement, py)?)?;
    permutation.add_function(wrap_pyfunction!(lehmer_code, py)?)?;
    permutation.add_function(wrap_pyfunction!(lexicographic_rank, py)?)?;
    permutation.add_function(wrap_pyfunction!(repr, py)?)?;

    m.add_submodule(&validators)?;
    m.add_submodule(&permutation)?;

    Ok(())
}
