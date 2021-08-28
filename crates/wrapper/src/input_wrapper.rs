use pyo3::prelude::*;

use pyxel::{Key, KeyValue};

use crate::instance;

#[pyfunction]
fn mouse(is_visible: bool) -> PyResult<()> {
    instance().mouse(is_visible);

    Ok(())
}

#[pyfunction]
fn btn(key: Key) -> PyResult<bool> {
    Ok(instance().btn(key))
}

#[pyfunction]
fn btnp(
    key: Key,
    hold_frame_count: Option<u32>,
    repeat_frame_count: Option<u32>,
) -> PyResult<bool> {
    Ok(instance().btnp(key, hold_frame_count, repeat_frame_count))
}

#[pyfunction]
fn btnr(key: Key) -> PyResult<bool> {
    Ok(instance().btnr(key))
}

#[pyfunction]
fn btnv(key: Key) -> PyResult<KeyValue> {
    Ok(instance().btnv(key))
}

pub fn add_input_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mouse, m)?)?;
    m.add_function(wrap_pyfunction!(btn, m)?)?;
    m.add_function(wrap_pyfunction!(btnp, m)?)?;
    m.add_function(wrap_pyfunction!(btnr, m)?)?;
    m.add_function(wrap_pyfunction!(btnv, m)?)?;

    Ok(())
}
