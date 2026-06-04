use crate::utils::{random_matrice, random_tableau};
use crate::utils::TypeProbleme;
use crate::tbwriter::TBLogger;

#[derive(Clone)]
pub enum Activation {
    Sigmoide,
    Tanh,
}

pub struct PMC {
    pub poids: Vec<Vec<Vec<f64>>>,
    pub biais: Vec<Vec<f64>>,
    pub txapprentissage: f64,
    pub activation: Activation,
    pub probleme: TypeProbleme,
    pub train_logger: Option<TBLogger>,
    pub test_logger: Option<TBLogger>,
}

impl PMC {
    pub fn new(
        couches: Vec<usize>,
        txapprentissage: f64,
        activation: Activation,
        probleme: TypeProbleme,
        train_logger: Option<TBLogger>,
        test_logger: Option<TBLogger>,
    ) -> Self {
        let mut poids = Vec::new();
        let mut biais = Vec::new();

        for i in 0..couches.len() - 1 {
            let matrice = random_matrice(vec![vec![0.0; couches[i]]; couches[i + 1]]);
            let tableau = random_tableau(vec![0.0; couches[i + 1]]);
            poids.push(matrice);
            biais.push(tableau);
        }

        Self { poids, biais, txapprentissage, activation, probleme, train_logger, test_logger }
    }

    pub fn activer(&self, x: f64) -> f64 {
        match self.activation {
            Activation::Sigmoide => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
        }
    }

    pub fn derivee_activation(&self, x: f64) -> f64 {
        match self.activation {
            Activation::Sigmoide => {
                let s = 1.0 / (1.0 + (-x).exp());
                s * (1.0 - s)
            }
            Activation::Tanh => {
                let t = x.tanh();
                1.0 - t * t
            }
        }
    }

    pub fn sauvegarder(&self, chemin: &str) -> std::io::Result<()> {
        let activation = match self.activation {
            Activation::Sigmoide => "sigmoide",
            Activation::Tanh => "tanh",
        };

        let probleme = match self.probleme {
            TypeProbleme::Classification => "classification",
            TypeProbleme::Regression => "regression",
        };

        let contenu = format!(
            r#"{{
"txapprentissage": {},
"activation": "{}",
"probleme": "{}",
"poids": {:?},
"biais": {:?}
}}"#,
            self.txapprentissage, activation, probleme, self.poids, self.biais
        );

        std::fs::write(chemin, contenu)
    }

    pub fn forward(&self, entree: &Vec<f64>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut activations = vec![entree.clone()];
        let mut zs = Vec::new();

        for (index_couche, (poids_couche, biais_couche)) in
            self.poids.iter().zip(self.biais.iter()).enumerate()
        {
            let activation_precedente = activations.last().unwrap();
            let mut z_couche = Vec::new();

            for (neurone_poids, biais_neurone) in poids_couche.iter().zip(biais_couche.iter()) {
                let mut somme = 0.0;
                for (poids, activation) in neurone_poids.iter().zip(activation_precedente.iter()) {
                    somme += poids * activation;
                }
                somme += biais_neurone;
                z_couche.push(somme);
            }

            let activation_couche = if index_couche == self.poids.len() - 1 {
                match self.probleme {
                    TypeProbleme::Classification => {
                        z_couche.iter().map(|z| self.activer(*z)).collect()
                    }
                    TypeProbleme::Regression => z_couche.clone(),
                }
            } else {
                z_couche.iter().map(|z| self.activer(*z)).collect()
            };

            zs.push(z_couche);
            activations.push(activation_couche);
        }

        (zs, activations)
    }

    // Calcule juste la loss sans modifier les poids
    fn calculer_loss(&self, dataset: &Vec<(Vec<f64>, Vec<f64>)>) -> f64 {
        let total_error: f64 = dataset.iter().map(|(x, y)| {
            let (_, activations) = self.forward(x);
            let sortie = activations.last().unwrap();
            sortie.iter().zip(y.iter())
                .map(|(pred, attendu)| (pred - attendu).powi(2))
                .sum::<f64>()
        }).sum();

        total_error / dataset.len() as f64
    }

    pub fn entrainer(
        &mut self,
        dataset_train: &Vec<(Vec<f64>, Vec<f64>)>,
        dataset_test: &Vec<(Vec<f64>, Vec<f64>)>,
        nbgeneration: usize,
    ) {
        for generation in 0..nbgeneration {
            let mut total_error = 0.0;

            let mut gradient_poids = self.poids.clone();
            let mut gradient_biais = self.biais.clone();

            for (gp, gb) in gradient_poids.iter_mut().zip(gradient_biais.iter_mut()) {
                for ligne in gp.iter_mut() {
                    for val in ligne.iter_mut() { *val = 0.0; }
                }
                for val in gb.iter_mut() { *val = 0.0; }
            }

            for (x, y) in dataset_train {
                let (zs, activations) = self.forward(x);
                let mut deltas = Vec::new();

                let y_erreur_courante = activations.last().unwrap();
                let delta_sortie: Vec<f64> = y_erreur_courante.iter()
                    .zip(y.iter())
                    .map(|(pred, attendu)| pred - attendu)
                    .collect();
                deltas.push(delta_sortie);

                let error: f64 = y_erreur_courante.iter()
                    .zip(y.iter())
                    .map(|(pred, attendu)| (pred - attendu).powi(2))
                    .sum();
                total_error += error;

                for l in (1..self.poids.len()).rev() {
                    let z = &zs[l - 1];
                    let w_suivant = &self.poids[l];
                    let delta_suivant = deltas.last().unwrap();

                    let delta: Vec<f64> = (0..z.len())
                        .map(|i| {
                            let somme: f64 = w_suivant.iter()
                                .zip(delta_suivant.iter())
                                .map(|(w_row, d)| w_row[i] * d)
                                .sum();
                            somme * self.derivee_activation(z[i])
                        })
                        .collect();
                    deltas.push(delta);
                }

                deltas.reverse();

                for l in 0..self.poids.len() {
                    for i in 0..self.poids[l].len() {
                        for j in 0..self.poids[l][i].len() {
                            gradient_poids[l][i][j] += deltas[l][i] * activations[l][j];
                        }
                        gradient_biais[l][i] += deltas[l][i];
                    }
                }
            }

            let m = dataset_train.len() as f64;

            for l in 0..self.poids.len() {
                for i in 0..self.poids[l].len() {
                    for j in 0..self.poids[l][i].len() {
                        self.poids[l][i][j] -= self.txapprentissage * gradient_poids[l][i][j] / m;
                    }
                    self.biais[l][i] -= self.txapprentissage * gradient_biais[l][i] / m;
                }
            }


            let train_loss = total_error / m;
            if let Some(logger) = &mut self.train_logger {
                logger.log_loss(generation, train_loss);
            }

            if !dataset_test.is_empty() {
                let test_loss = self.calculer_loss(dataset_test);
                if let Some(logger) = &mut self.test_logger {
                    logger.log_loss(generation, test_loss);
                }
            }
        }

        if let Some(logger) = &mut self.train_logger {
            logger.flush();
        }
        if let Some(logger) = &mut self.test_logger {
            logger.flush();
        }
    }

    pub fn predire(&self, x: &Vec<f64>) -> Vec<f64> {
        let (_, activations) = self.forward(x);
        let sortie = activations.last().unwrap();

        match self.probleme {
            TypeProbleme::Classification => {
                sortie.iter().map(|v| if *v > 0.0 { 1.0 } else { -1.0 }).collect()
            }
            TypeProbleme::Regression => sortie.clone(),
        }
    }
}