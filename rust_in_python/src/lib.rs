use pyo3::prelude::*;

#[pyfunction]
pub fn fibo(n: usize) -> usize {
    match n {
        1 => 1,
        2 => 1,
        _ => fibo(n - 2) + fibo(n - 1)
    }
}

//
// #[pyfunction]
// pub fn fibors(n: usize) -> usize {
//     fibo(n)
// }


#[pymodule]
pub fn fiborust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibo, m)?)?;
    Ok(())
}
