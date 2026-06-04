use rand::Rng;

pub fn random_matrice(mut matrice: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    for i in 0..matrice.len() {
        for j in 0..matrice[i].len() {
            matrice[i][j] = rng.gen_range(-0.5..0.5);
        }
    }
    matrice
}

pub fn random_tableau(mut tableau: Vec<f64>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    for i in 0..tableau.len() {
        tableau[i] = rng.gen_range(-0.5..0.5);
    }
    tableau
}

pub fn tanh_derivative(x: f64) -> f64 {
    let t = x.tanh();
    1.0 - t * t
}

pub fn oracle(x: f64, y: f64) -> f64 {
    (x as i64 ^ y as i64) as f64
}

pub enum TypeProbleme {
    Classification,
    Regression,
}