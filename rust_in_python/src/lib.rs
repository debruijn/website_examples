use pyo3::prelude::*;

#[pyfunction]
pub fn fibo(n: usize) -> usize {
    if n == 1 || n == 2 {
        1
    } else {
        fibo(n - 2) + fibo(n - 1)
    }
}


#[pymodule]
#[pyo3(name = "fibo_rust")]
pub fn fiborust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibo, m)?)?;
    Ok(())
}
