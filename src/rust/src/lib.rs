use extendr_api::prelude::*;
use ndarray_linalg::Inverse;

/// @export
#[extendr]
fn multiplicame_e_invierteme(
    // ArrayView2<f64> es una vista la matriz (inmutable)
    m1: ArrayView2<f64>,
    m2: ArrayView2<f64>,
) -> Result<Robj> {
    // multiplicamos
    let multiplicacion = m1.dot(&m2);
    // Invertimos y la intemamos convertir a una matriz de R
    multiplicacion.inv().expect("Unable to invert").try_into()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod mymulti;
    fn multiplicame_e_invierteme;
}
