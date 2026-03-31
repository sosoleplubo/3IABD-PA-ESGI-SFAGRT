use pyo3::prelude::*;
use rand::Rng;

#[pyfunction]
pub fn genetique(
    py: Python,
    oracle: PyObject,
    taille_population: usize,
    nb_elites: usize,
    nb_generation: i64,
    domaine_min: i64,
    domaine_max: i64,
) -> PyResult<i64> {

    #[derive(Clone)]
    struct Individu {
        valeur: i64,
        score: i64,
    }

    let mut population: Vec<Individu> = Vec::new();
    let mut rng = rand::thread_rng();


    for _ in 0..taille_population {
        let tmp: i64 = rng.gen_range(domaine_min..domaine_max);

        let score: i64 = oracle.call1(py, (tmp,))?.extract(py)?;

        population.push(Individu {
            valeur: tmp,
            score,
        });
    }


    for generation in 0..nb_generation {
        population.sort_by_key(|individu| individu.score);

        println!("Gen {} → best = {}", generation, population[0].valeur);

        let meilleurs = &population[..nb_elites];
        let mut newpopulation: Vec<Individu> = Vec::new();

        // élites
        for i in 0..nb_elites {
            newpopulation.push(meilleurs[i].clone());
        }


        for _ in 0..(taille_population - nb_elites) {
            let papa = &meilleurs[rng.gen_range(0..nb_elites)];
            let maman = &meilleurs[rng.gen_range(0..nb_elites)];

            let alpha: f64 = rng.r#gen();
            let mut fiston = (alpha * papa.valeur as f64
                + (1.0 - alpha) * maman.valeur as f64) as i64;

            let facteur = 1.0 - (generation as f64 / nb_generation as f64);
            let amplitude = (10.0 * facteur) as i64;

            let mutation = rng.gen_range(-amplitude..=amplitude);
            fiston += mutation;

            let score: i64 = oracle.call1(py, (fiston,))?.extract(py)?;

            newpopulation.push(Individu {
                valeur: fiston,
                score,
            });
        }

        population = newpopulation;
    }

    population.sort_by_key(|individu| individu.score);

    Ok(population[0].valeur)
}
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
    m.add_function(wrap_pyfunction!(genetique, m)?)?;
    Ok(())
}