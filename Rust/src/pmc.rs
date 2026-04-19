use crate::utils::{random_matrice, random_tableau};

#[derive(Clone)]
pub enum Activation {
    Sigmoide,
    Tanh,
}

#[derive(Clone)]
pub enum TypeProbleme {
    Classification,
    Regression,
}

pub struct PMC {
    pub poids: Vec<Vec<Vec<f64>>>,
    pub biais: Vec<Vec<f64>>,
    pub txapprentissage: f64,
    pub activation: Activation,
    pub probleme: TypeProbleme,
}

impl PMC {
    pub fn new(
        couches: Vec<usize>,
        txapprentissage: f64,
        activation: Activation,
        probleme: TypeProbleme,
    ) -> Self {
        let mut poids = Vec::new();
        let mut biais = Vec::new();

        for i in 0..couches.len() - 1 {
            let matrice = random_matrice(vec![vec![0.0; couches[i]]; couches[i + 1]]);
            let tableau = random_tableau(vec![0.0; couches[i + 1]]);
            poids.push(matrice);
            biais.push(tableau);
        }

        Self { poids, biais, txapprentissage, activation, probleme }
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

    pub fn forward(&self, entree: &Vec<f64>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut activations = vec![entree.clone()];
        let mut zs = Vec::new();

        for (index_couche, (poids_couche, biais_couche)) in
            self.poids.iter().zip(self.biais.iter()).enumerate()
        {
            let activation_precedente = activations.last().unwrap();
            let mut z_couche = Vec::new();

            for (neurone_poids, biais_neurone) in
                poids_couche.iter().zip(biais_couche.iter())
            {
                let mut somme = 0.0;
                for (poids, activation) in
                    neurone_poids.iter().zip(activation_precedente.iter())
                {
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

    pub fn entrainer(&mut self, dataset: &Vec<(Vec<f64>, Vec<f64>)>, nbgeneration: usize) {
        for _ in 0..nbgeneration {
            let donnees = dataset.clone();

            let mut gradient_poids = self.poids.clone();
            let mut gradient_biais = self.biais.clone();

            for (gp, gb) in gradient_poids.iter_mut().zip(gradient_biais.iter_mut()) {
                for ligne in gp.iter_mut() {
                    for val in ligne.iter_mut() { *val = 0.0; }
                }
                for val in gb.iter_mut() { *val = 0.0; }
            }

            for (x, y) in &donnees {
                let (zs, activations) = self.forward(x);
                let mut deltas = Vec::new();

                let y_erreur_courante = activations.last().unwrap();
                let delta_sortie: Vec<f64> = y_erreur_courante.iter()
                    .zip(y.iter())
                    .map(|(pred, attendu)| pred - attendu)
                    .collect();
                deltas.push(delta_sortie);

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

            let m = donnees.len() as f64;
            for l in 0..self.poids.len() {
                for i in 0..self.poids[l].len() {
                    for j in 0..self.poids[l][i].len() {
                        self.poids[l][i][j] -= self.txapprentissage * gradient_poids[l][i][j] / m;
                    }
                    self.biais[l][i] -= self.txapprentissage * gradient_biais[l][i] / m;
                }
            }
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