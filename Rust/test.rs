use std::fs;

impl PMC {

    pub fn charger(chemin: &str) -> Self {
        let contenu = std::fs::read_to_string(chemin).unwrap();

        let activation = if contenu.contains("\"activation\": \"sigmoide\"") {
            Activation::Sigmoide
        } else {
            Activation::Tanh
        };

        let probleme = if contenu.contains("\"probleme\": \"classification\"") {
            TypeProbleme::Classification
        } else {
            TypeProbleme::Regression
        };

        // ici il faudrait parser poids et biais
        unimplemented!()
    }