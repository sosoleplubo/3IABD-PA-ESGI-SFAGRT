use rand::Rng;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn random_matrice(mut matrice : Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    for i in 0..matrice.len() {
        for j in 0..matrice[i].len() {
            matrice[i][j] = rng.gen_range(-0.5..0.5);
        }
    }
    return matrice;
}

fn random_tableau(mut tableau : Vec<f64>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    for i in 0..tableau.len() {
        tableau[i]=rng.gen_range(-0.5..0.5);
    }
    return tableau;
}

fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

fn oracle(x: f64, y:f64) -> f64 {
    return (x as i64^y as i64) as f64
}



struct PMC {
    w1: Vec<Vec<f64>>,
    b1: Vec<f64>,
    w2: Vec<f64>,
    b2: f64,
    learning_rate: f64,
}

impl PMC {
    fn new(input_size: usize, hidden_size: usize, lr: f64) -> Self {
        let mut rng = rand::thread_rng();

        let mut w1 = vec![vec![0.0; input_size]; hidden_size];

        w1 = random_matrice(w1);

        let mut b1 = vec![0.0; hidden_size];

        b1 = random_tableau(b1);

        let mut w2 = vec![0.0; hidden_size];

        w2 = random_tableau(w2);

        let mut b2 = rng.gen_range(-0.5..0.5);


        Self { w1, b1, w2, b2, learning_rate: lr }

    }
}

impl PMC {
    fn forward(&self, x: &Vec<f64>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut activations = vec![x.clone()];
        let mut valeuravactivation = Vec::new();

        (valeuravactivation, activations)
    }
}

impl PMC {
    fn train(&mut self, dataset: &Vec<(Vec<f64>, f64)>, nbgeneration: i64) {
        for _ in 0..nbgeneration {
            for (x, y) in dataset {

                let (valeuravactivation, activations) = self.forward(x);

                let mut deltas = Vec::new(); //stockage de chaque couche

                let y_hat = activations.last().unwrap()[0];
                let delta = (y_hat - y) * y_hat * (1.0 - y_hat);

                deltas.push(vec![delta]); //“stockage l’erreur de la couche de sortie”


            }
        }
    }
}

impl PMC {
    fn predict(&self, x: &Vec<f64>) -> f64 {
        let (_, activations) = self.forward(x);
        activations.last().unwrap()[0]
    }
}


fn main() {
    let dataset = vec![
        (vec![0.1, 0.2, 0.1], 0.0),
        (vec![0.2, 0.3, 0.2], 0.0),
        (vec![0.6, 0.6, 0.6], 1.0),
        (vec![0.9, 0.2, 0.5], 1.0),
        (vec![0.3, 0.3, 0.3], 0.0),
        (vec![0.7, 0.7, 0.2], 1.0),
    ];

    let mut model = PMC::new(3, 4, 0.1);

    model.train(&dataset, 100000);

    for (x, _) in &dataset {
        println!("{:?} -> {}", x, model.predict(x));
    }
}