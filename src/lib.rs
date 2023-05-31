use chrono::Utc;
use cron::Schedule;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::str::FromStr;

/// validate cron expression
#[pyfunction]
#[pyo3(signature = (expression))]
fn check_cron_expression(expression: String) -> PyResult<String> {
    let result = match Schedule::from_str(expression.as_str()) {
        Ok(_) => String::new(),
        Err(error) => format!("Check `{}` failed, due to {:?}", &expression, error),
    };
    Ok(result)
}

// class
#[pyclass]
struct CronExpr {
    #[pyo3(get)]
    source: String,
    schedule: Schedule,
}

#[pymethods]
impl CronExpr {
    /// crate::cron::Schedule wrapper class
    /// NOTE: will raise ValueError when wrong input
    #[new]
    #[pyo3(text_signature = "(expression)")]
    fn py_new(expression: String) -> PyResult<Self> {
        let err_msg = match Schedule::from_str(expression.as_str()) {
            Ok(_) => String::new(),
            Err(error) => format!("Check `{}` failed, due to {:?}", &expression, error),
        };
        if !err_msg.is_empty() {
            Err(PyValueError::new_err(err_msg))
        } else {
            Ok(Self {
                source: expression.clone(),
                schedule: Schedule::from_str(expression.as_str()).unwrap(),
            })
        }
    }
    /// Get next n upcomming time
    /// Example: ['2022-09-02 21:01:00 UTC', '2022-09-03 21:01:00 UTC', '2022-09-04 21:01:00 UTC', '2022-09-05 21:01:00 UTC', '2022-09-06 21:01:00 UTC']
    #[pyo3(text_signature = "($self, n)")]
    fn next_n_upcoming_time_literals(&self, n: usize) -> PyResult<Vec<String>> {
        Ok(self
            .schedule
            .upcoming(Utc)
            .take(n)
            .map(|dt| format!("{}", dt))
            .collect())
    }
}

#[pymodule]
fn cro3n(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_cron_expression, m)?)?;
    m.add_class::<CronExpr>()?;
    Ok(())
}
