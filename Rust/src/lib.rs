

use pyo3::prelude::*;



#[pyclass]
struct ModeleRegressLin {
    modele: RegressionLineaire,
}

mod utils;
mod pmc;
mod genetique;
mod recuitsimule;
mod regresslin;
mod tbwriter;

use crate::tbwriter::TBLogger;
use regresslin::RegressionLineaire;
use crate::utils::TypeProbleme;
use pyo3::prelude::*;
use pmc::{PMC, Activation};

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
        logdir_train: Option<String>,
        logdir_test: Option<String>,
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

        let train_logger = logdir_train.map(|dir| TBLogger::new(&dir));
        let test_logger  = logdir_test.map(|dir| TBLogger::new(&dir));

        Ok(Self {
            modele: PMC::new(couches, txapprentissage, act, prob, train_logger, test_logger),
        })
    }

    fn entrainer(
        &mut self,
        dataset_train: Vec<(Vec<f64>, Vec<f64>)>,
        dataset_test: Vec<(Vec<f64>, Vec<f64>)>,
        nbgeneration: usize,
    ) {
        self.modele.entrainer(&dataset_train, &dataset_test, nbgeneration);
    }

    fn predire(&self, x: Vec<f64>) -> Vec<f64> {
        self.modele.predire(&x)
    }

    fn sauvegarder(&self, chemin: String) -> PyResult<()> {
        self.modele
            .sauvegarder(&chemin)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}

#[pymethods]
impl ModeleRegressLin {

    #[new]
    fn new(nb_features: usize, probleme: &str) -> PyResult<Self> {

        let prob = match probleme {
            "classification" => TypeProbleme::Classification,
            "regression" => TypeProbleme::Regression,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(
                "probleme doit être classification ou regression"
            )),
        };

        Ok(Self {
            modele: RegressionLineaire::new(nb_features, prob),
        })
    }

    fn entrainer(&mut self, dataset: Vec<(Vec<f64>, Vec<f64>)>) {
        self.modele.entrainer(&dataset);
    }

    fn predire(&self, x: Vec<f64>) -> Vec<f64> {
        self.modele.predire(&x)
    }

    fn predire_brut(&self, x: Vec<f64>) -> f64 {
        self.modele.predire_brut(&x)
    }

    fn sauvegarder(&self, chemin: String) -> PyResult<()> {
        self.modele
            .sauvegarder(&chemin)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}

#[pymodule]
fn PAESGISFAGRT(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ModelePMC>()?;
    m.add_class::<ModeleRegressLin>()?;
    Ok(())
}