mod utils;
mod pmc;

use pyo3::prelude::*;
use pmc::{PMC, Activation, TypeProbleme};

#[pyclass]
struct ModelePMC {
    modele: PMC,
}

#[pymethods]
impl ModelePMC {

    #[new]
    fn new(
        couches: Vec<usize>,
        txapprentissage: f64,
        activation: &str,
        probleme: &str,
    ) -> PyResult<Self> {

        let act = match activation {
            "sigmoide" => Activation::Sigmoide,
            "tanh"     => Activation::Tanh,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(
                "activation doit être 'sigmoide' ou 'tanh'"
            )),
        };

        let prob = match probleme {
            "classification" => TypeProbleme::Classification,
            "regression"     => TypeProbleme::Regression,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(
                "probleme doit être 'classification' ou 'regression'"
            )),
        };

        Ok(Self {
            modele: PMC::new(couches, txapprentissage, act, prob),
        })
    }

    fn entrainer(&mut self, dataset: Vec<(Vec<f64>, Vec<f64>)>, nbgeneration: usize) {
        self.modele.entrainer(&dataset, nbgeneration);
    }

    fn predire(&self, x: Vec<f64>) -> Vec<f64> {
        self.modele.predire(&x)
    }
}

#[pymodule]
fn PAESGISFAGRT(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ModelePMC>()?;
    Ok(())
}