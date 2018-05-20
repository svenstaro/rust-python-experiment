#![feature(proc_macro, specialization)]

extern crate pyo3;
extern crate rayon;
use pyo3::prelude::*;

use pyo3::py::modinit as pymodinit;
use rayon::prelude::*;

// Add bindings to the generated python module
// N.B: names: "librust_python_experiment" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodinit(myrustpylib)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    // ``#[pyfn()]` converts the arguments from Python objects to Rust values
    // and the Rust return value back into a Python object.
    #[pyfn(m, "sum_as_string")]
    fn sum_as_string_py(a:i64, b:i64) -> PyResult<String> {
       let out = sum_as_string(a, b);
       Ok(out)
    }

    Ok(())
}

// The logic can be implemented as a normal rust function
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}
