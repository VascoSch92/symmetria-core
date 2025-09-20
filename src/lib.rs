mod elements;
use crate::elements::cycle;
use crate::elements::permutation;
use crate::elements::validators;

use pyo3::prelude::*;

#[pymodule]
fn _symmetria_core(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let table = PyModule::new(py, "table")?;
    table.add_class::<crate::elements::table::Table>()?;

    let validators_module = PyModule::new(py, "validators")?;
    validators_module.add_function(wrap_pyfunction!(validators::validate_permutation, py)?)?;
    validators_module.add_function(wrap_pyfunction!(validators::validate_cycle, py)?)?;
    validators_module.add_function(wrap_pyfunction!(
        validators::validate_cycle_decomposition,
        py
    )?)?;

    let permutation_module = PyModule::new(py, "permutation")?;
    permutation_module.add_function(wrap_pyfunction!(permutation::ascents, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::call_on_int, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::call_on_str, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::descents, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::exceedances, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::int_repr, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::inversions, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::is_derangement, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::lehmer_code, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::lexicographic_rank, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::repr, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::multiplication, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::records, py)?)?;
    permutation_module.add_function(wrap_pyfunction!(permutation::support, py)?)?;

    let cycle_module = PyModule::new(py, "cycle")?;
    cycle_module.add_function(wrap_pyfunction!(cycle::call_on_int, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::call_on_str, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::int_repr, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::inversions, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::map, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::repr, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::standardization, py)?)?;
    cycle_module.add_function(wrap_pyfunction!(cycle::str, py)?)?;

    m.add_submodule(&table)?;
    m.add_submodule(&validators_module)?;
    m.add_submodule(&permutation_module)?;
    m.add_submodule(&cycle_module)?;

    Ok(())
}
