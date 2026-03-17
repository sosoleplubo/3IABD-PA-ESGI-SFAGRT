use pyo3::prelude::*;
use rand::Rng;

#[pyfunction]
pub fn recuitsimule(py: Python, oracle: PyObject, tmin: f64, t_init: f64,
    alpha: f64,
) -> PyResult<i64> {

    let mut t: f64 = t_init;

    let mut solution: i64 = 0;
    let mut counter: i64 = 0;
    let mut m_solution: i64 = solution;

    let mut rng = rand::thread_rng();

    while t > tmin {

        let pas: i64 = rng.gen_range(-7..7);
        println!("solution : {}", solution);

        let voisin: i64 = solution + pas;
        println!("voisin : {}", voisin);


        let val_solution: i64 = oracle.call1(py, (solution,))?.extract(py)?;
        let val_voisin: i64 = oracle.call1(py, (voisin,))?.extract(py)?;

        let diff = (val_voisin - val_solution) as f64;

        if diff < 0.0 {
            solution = voisin;
        } else {
            let p = (-diff / t).exp();
            if rng.r#gen::<f64>() < p {
                solution = voisin;
            }
        }

        // 🔥 update meilleur (comme ton code)
        let val_m: i64 = oracle.call1(py, (m_solution,))?.extract(py)?;
        let val_current: i64 = oracle.call1(py, (solution,))?.extract(py)?;

        if val_current < val_m {
            m_solution = solution;
        }

        println!("temperature : {}", t);

        t *= alpha;
        counter += 1;
    }

    println!("compteur : {}", counter);

    Ok(m_solution)
}

#[pymodule]
fn PAESGISFAGRT(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(recuitsimule, m)?)?;
    Ok(())
}