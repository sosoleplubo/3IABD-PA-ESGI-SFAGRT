use nalgebra::{DMatrix, DVector};
use crate::utils::TypeProbleme;


pub struct RegressionLineaire {
    pub poids: Vec<f64>,
    pub biais: f64,
    pub probleme: TypeProbleme,
}

impl RegressionLineaire {
    pub fn new(nb_features: usize, probleme: TypeProbleme) -> Self {
        Self {
            poids: vec![0.0; nb_features],
            biais: 0.0,
            probleme,
        }
    }

    pub fn entrainer(&mut self, dataset: &Vec<(Vec<f64>, Vec<f64>)>) {

        let nb_lignes = dataset.len();

        let nb_features = dataset[0].0.len();

        let mut x_data = Vec::with_capacity(nb_lignes * (nb_features + 1));

        let mut y_data = Vec::with_capacity(nb_lignes);

        for (x, y) in dataset {

            x_data.push(1.0); // biais

            for v in x {

                x_data.push(*v);

            }

            y_data.push(y[0]);

        }

        let x = DMatrix::from_row_slice(nb_lignes, nb_features + 1, &x_data);

        let y = DVector::from_row_slice(&y_data);

        // X^T

        let xt = x.transpose();

        // X^T X

        let xtx = &xt * &x;

        // inverse

        let xtx_inv = xtx.try_inverse().expect("Matrix non inversible");

        // X^T Y

        let xty = &xt * &y;

        // W

        let w = xtx_inv * xty;

        self.biais = w[0];

        for i in 0..nb_features {

            self.poids[i] = w[i + 1];

        }

    }

    pub fn predire_brut(&self, x: &Vec<f64>) -> f64 {
        let mut somme = self.biais;

        for (poids, valeur) in self.poids.iter().zip(x.iter()) {
            somme += poids * valeur;
        }

        somme
    }

    pub fn predire(&self, x: &Vec<f64>) -> Vec<f64> {
        let y = self.predire_brut(x);

        match self.probleme {
            TypeProbleme::Regression => vec![y],

            TypeProbleme::Classification => {
                if y >= 0.0 {
                    vec![1.0]
                } else {
                    vec![-1.0]
                }
            }
        }
    }

    pub fn sauvegarder(&self, chemin: &str) -> std::io::Result<()> {
        let probleme = match self.probleme {
            TypeProbleme::Classification => "classification",
            TypeProbleme::Regression => "regression",
        };

        let contenu = format!(
            r#"{{
"probleme": "{}",
"poids": {:?},
"biais": {}
}}"#,
            probleme,
            self.poids,
            self.biais
        );

        std::fs::write(chemin, contenu)
    }
}