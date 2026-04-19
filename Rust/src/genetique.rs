use pyo3::prelude::*;
use rand::Rng;

#[derive(Clone)]
struct Individu {
    valeur: i64,
    score: i64,
}

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
    let mut population: Vec<Individu> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..taille_population {
        let tmp: i64 = rng.gen_range(domaine_min..domaine_max);
        let score: i64 = oracle.call1(py, (tmp,))?.extract(py)?;
        population.push(Individu { valeur: tmp, score });
    }

    for generation in 0..nb_generation {
        population.sort_by_key(|individu| individu.score);
        println!("Gen {} → best = {}", generation, population[0].valeur);

        let meilleurs = &population[..nb_elites];
        let mut newpopulation: Vec<Individu> = Vec::new();

        for i in 0..nb_elites {
            newpopulation.push(meilleurs[i].clone());
        }

        for _ in 0..(taille_population - nb_elites) {
            let papa = &meilleurs[rng.gen_range(0..nb_elites)];
            let maman = &meilleurs[rng.gen_range(0..nb_elites)];

            let alpha: f64 = rng.r#gen();
            let mut fiston =
                (alpha * papa.valeur as f64 + (1.0 - alpha) * maman.valeur as f64) as i64;

            let facteur = 1.0 - (generation as f64 / nb_generation as f64);
            let amplitude = (10.0 * facteur) as i64;
            let mutation = rng.gen_range(-amplitude..=amplitude);
            fiston += mutation;

            let score: i64 = oracle.call1(py, (fiston,))?.extract(py)?;
            newpopulation.push(Individu { valeur: fiston, score });
        }

        population = newpopulation;
    }

    population.sort_by_key(|individu| individu.score);
    Ok(population[0].valeur)
}