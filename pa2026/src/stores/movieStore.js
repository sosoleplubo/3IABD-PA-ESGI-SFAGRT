import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useMovieStore = defineStore('movie', () => {
  const predictions = ref([])
  const currentForm = ref({
    title: '',
    genre: '',
    budget: '',
    director_known: false,
    star_power: 3,
    sequel: false,
    season: '',
    marketing_budget: '',
    runtime: '',
  })

  // Modèle de prédiction simplifié (heuristique)
  function predictBoxOffice(form) {
    let base = parseFloat(form.budget) || 0
    let multiplier = 2.5

    // Genre
    const genreMultipliers = {
      action: 3.2, animation: 3.8, superhero: 4.5, horror: 3.0,
      comedy: 2.2, drama: 1.8, scifi: 3.5, thriller: 2.5
    }
    multiplier *= (genreMultipliers[form.genre] || 2.5) / 2.5

    // Star power (1-5)
    multiplier *= 0.7 + (form.star_power * 0.15)

    // Réalisateur connu
    if (form.director_known) multiplier *= 1.25

    // Sequel
    if (form.sequel) multiplier *= 1.35

    // Saison de sortie
    const seasonMultipliers = { summer: 1.4, holiday: 1.3, spring: 1.0, fall: 1.1 }
    multiplier *= seasonMultipliers[form.season] || 1.0

    // Marketing
    const marketingBoost = (parseFloat(form.marketing_budget) || 0) * 0.8
    const rawPrediction = base * multiplier + marketingBoost

    // Score de confiance
    const confidence = Math.min(95, 55 + (form.star_power * 5) + (form.director_known ? 10 : 0) + (form.sequel ? 8 : 0))

    return {
      amount: rawPrediction,
      confidence,
      roi: base > 0 ? ((rawPrediction - base) / base * 100).toFixed(1) : 0,
      tier: rawPrediction > 500 ? 'Blockbuster' : rawPrediction > 150 ? 'Hit' : rawPrediction > 50 ? 'Modéré' : 'Risqué'
    }
  }

  function addPrediction() {
    const result = predictBoxOffice(currentForm.value)
    predictions.value.unshift({
      id: Date.now(),
      ...currentForm.value,
      ...result,
      date: new Date().toLocaleDateString('fr-FR')
    })
  }

  function removePrediction(id) {
    predictions.value = predictions.value.filter(p => p.id !== id)
  }

  const avgRoi = computed(() => {
    if (!predictions.value.length) return 0
    return (predictions.value.reduce((a, p) => a + parseFloat(p.roi), 0) / predictions.value.length).toFixed(1)
  })

  const topFilm = computed(() => {
    if (!predictions.value.length) return null
    return predictions.value.reduce((a, b) => a.amount > b.amount ? a : b)
  })

  return { predictions, currentForm, addPrediction, removePrediction, predictBoxOffice, avgRoi, topFilm }
})
