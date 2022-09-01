use cron::Schedule;
use pyo3::prelude::*;
use std::str::FromStr;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn check_cron_expression(expression: String) -> PyResult<String> {
    let result = match Schedule::from_str(expression.as_str()) {
        Ok(_) => String::new(),
        Err(error) => format!("Check `{}` failed, due to {:?}", &expression, error),
    };
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn cro3n(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_cron_expression, m)?)?;
    Ok(())
}
