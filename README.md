# 3IABD-PA-ESGI-SFAGRT

# Movie Box Office Prediction

## 🎯 Objectif
Prédire le box-office d’un film avant sa sortie à partir de son casting
et de métadonnées (budget, genre, studio, date de sortie).

Projet réalisé dans le cadre du **Projet Annuel 3 – Big Data (2025–2026)**.

---

## 🧠 Problématique
Le succès commercial d’un film dépend de nombreux facteurs difficiles
à modéliser manuellement. Ce projet explore l’utilisation de modèles
de Machine Learning pour estimer le box-office à partir d’informations
connues avant la sortie du film.

---

## Données
- Source principale : TMDB/IMDB
- Variables utilisées :
  - Casting (star power)
  - Budget
  - Genre(s)
  - Studio
  - Date de sortie
  - Durée

La variable cible est le **box-office worldwide (log-transformé)**.

---

## Modèles implémentés
Tous les modèles sont **implémentés manuellement en C++**, puis utilisés
via des bindings Python.

- Modèle linéaire
- Perceptron Multi-Couches (MLP)
- Réseau à Fonctions de Base Radiale (RBF)
- SVM (comparaison)

---
