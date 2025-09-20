mod elements;

use crate::elements::permutation::{
    ascents, call_on_int, call_on_str, descents, exceedances, int_repr, inversions, is_derangement,
    lehmer_code, lexicographic_rank, multiplication, records, repr, support,
};
use crate::elements::validators::{
    validate_cycle, validate_cycle_decomposition, validate_permutation,
};

use pyo3::prelude::*;

#[pymodule]
fn _symmetria_core(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let table = PyModule::new(py, "table")?;
    table.add_class::<crate::elements::table::Table>()?;

    let validators = PyModule::new(py, "validators")?;
    validators.add_function(wrap_pyfunction!(validate_permutation, py)?)?;
    validators.add_function(wrap_pyfunction!(validate_cycle, py)?)?;
    validators.add_function(wrap_pyfunction!(validate_cycle_decomposition, py)?)?;

    let permutation = PyModule::new(py, "permutation")?;
    permutation.add_function(wrap_pyfunction!(ascents, py)?)?;
    permutation.add_function(wrap_pyfunction!(call_on_int, py)?)?;
    permutation.add_function(wrap_pyfunction!(call_on_str, py)?)?;
    permutation.add_function(wrap_pyfunction!(descents, py)?)?;
    permutation.add_function(wrap_pyfunction!(exceedances, py)?)?;
    permutation.add_function(wrap_pyfunction!(int_repr, py)?)?;
    permutation.add_function(wrap_pyfunction!(inversions, py)?)?;
    permutation.add_function(wrap_pyfunction!(is_derangement, py)?)?;
    permutation.add_function(wrap_pyfunction!(lehmer_code, py)?)?;
    permutation.add_function(wrap_pyfunction!(lexicographic_rank, py)?)?;
    permutation.add_function(wrap_pyfunction!(repr, py)?)?;
    permutation.add_function(wrap_pyfunction!(multiplication, py)?)?;
    permutation.add_function(wrap_pyfunction!(records, py)?)?;
    permutation.add_function(wrap_pyfunction!(support, py)?)?;

    m.add_submodule(&table)?;
    m.add_submodule(&validators)?;
    m.add_submodule(&permutation)?;

    Ok(())
}
