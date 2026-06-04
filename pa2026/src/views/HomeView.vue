<script setup>
import { ref } from 'vue'
import { useMovieStore } from '@/stores/movieStore'
import { useRouter } from 'vue-router'

const store = useMovieStore()
const router = useRouter()
const preview = ref(null)
const submitted = ref(false)

const selectStyle = `
  width: 100%; padding: 0.65rem 1rem; border-radius: 10px;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.1);
  color: #fff; font-size: 0.95rem; outline: none;
  box-sizing: border-box; cursor: pointer;
`

const options = {
  title: [
    { value: '', label: 'Choisir un film...' },
    { value: 'Avatar 3', label: 'Avatar 3' },
    { value: 'Avengers Secret Wars', label: 'Avengers: Secret Wars' },
    { value: 'Fast & Furious 11', label: 'Fast & Furious 11' },
    { value: 'Mission Impossible 8', label: 'Mission: Impossible 8' },
    { value: 'Jurassic World 4', label: 'Jurassic World 4' },
    { value: 'The Batman 2', label: 'The Batman 2' },
    { value: 'Inception 2', label: 'Inception 2' },
    { value: 'Dune Part 3', label: 'Dune: Part Three' },
  ],
  studio: [
    { value: '', label: 'Choisir un studio...' },
    { value: 'disney', label: '🏰 Disney / Marvel' },
    { value: 'wb', label: '🛡️ Warner Bros.' },
    { value: 'universal', label: '🌍 Universal Pictures' },
    { value: 'paramount', label: '⛰️ Paramount Pictures' },
    { value: 'sony', label: '🎬 Sony Pictures' },
    { value: 'netflix', label: '🔴 Netflix' },
    { value: 'a24', label: '🎭 A24' },
  ],
  genre: [
    { value: '', label: 'Choisir un genre...' },
    { value: 'action', label: '💥 Action' },
    { value: 'animation', label: '🎨 Animation' },
    { value: 'superhero', label: '🦸 Super-héros' },
    { value: 'horror', label: '👻 Horreur' },
    { value: 'comedy', label: '😂 Comédie' },
    { value: 'drama', label: '🎭 Drame' },
    { value: 'scifi', label: '🚀 Sci-Fi' },
    { value: 'thriller', label: '🔪 Thriller' },
  ],
  director: [
    { value: '', label: 'Choisir un réalisateur...' },
    { value: 'james_cameron', label: 'James Cameron' },
    { value: 'christopher_nolan', label: 'Christopher Nolan' },
    { value: 'steven_spielberg', label: 'Steven Spielberg' },
    { value: 'denis_villeneuve', label: 'Denis Villeneuve' },
    { value: 'ridley_scott', label: 'Ridley Scott' },
    { value: 'greta_gerwig', label: 'Greta Gerwig' },
    { value: 'ryan_coogler', label: 'Ryan Coogler' },
    { value: 'inconnu', label: 'Réalisateur inconnu' },
  ],
  lead_actor: [
    { value: '', label: 'Choisir un acteur principal...' },
    { value: 'tom_cruise', label: 'Tom Cruise' },
    { value: 'dwayne_johnson', label: 'Dwayne Johnson' },
    { value: 'scarlett_johansson', label: 'Scarlett Johansson' },
    { value: 'ryan_reynolds', label: 'Ryan Reynolds' },
    { value: 'zendaya', label: 'Zendaya' },
    { value: 'timothee_chalamet', label: 'Timothée Chalamet' },
    { value: 'margot_robbie', label: 'Margot Robbie' },
    { value: 'inconnu', label: 'Acteur inconnu' },
  ],
  budget: [
    { value: '', label: 'Choisir un budget...' },
    { value: '20', label: '< 20M$ (Petit budget)' },
    { value: '50', label: '20–50M$ (Indépendant)' },
    { value: '100', label: '50–100M$ (Moyen)' },
    { value: '175', label: '100–200M$ (Blockbuster)' },
    { value: '275', label: '200–300M$ (Méga production)' },
    { value: '400', label: '300M$+ (Titan)' },
  ],
  marketing_budget: [
    { value: '', label: 'Choisir un budget marketing...' },
    { value: '10', label: '< 10M$ (Minimal)' },
    { value: '30', label: '10–50M$ (Standard)' },
    { value: '80', label: '50–100M$ (Intensif)' },
    { value: '150', label: '100–200M$ (Massif)' },
    { value: '250', label: '200M$+ (Mondial)' },
  ],
  runtime: [
    { value: '', label: 'Choisir une durée...' },
    { value: '85', label: '< 90 min (Court)' },
    { value: '105', label: '90–120 min (Standard)' },
    { value: '135', label: '120–150 min (Long)' },
    { value: '160', label: '150–180 min (Épique)' },
    { value: '200', label: '180 min+ (Monumental)' },
  ],
  season: [
    { value: '', label: 'Choisir une saison...' },
    { value: 'summer', label: '☀️ Été (juin–août)' },
    { value: 'holiday', label: '🎄 Fêtes (nov–déc)' },
    { value: 'spring', label: '🌸 Printemps (mars–mai)' },
    { value: 'fall', label: '🍂 Automne (sept–oct)' },
  ],
  star_power: [
    { value: 1, label: '⚪ 1 — Inconnu' },
    { value: 2, label: '🟡 2 — Emergent' },
    { value: 3, label: '🟠 3 — Reconnu' },
    { value: 4, label: '🔴 4 — Star confirmée' },
    { value: 5, label: '⭐ 5 — Superstar mondiale' },
  ],
  sequel: [
    { value: false, label: '🆕 Film original' },
    { value: true, label: '🔁 Suite / Franchise' },
  ],
  director_known: [
    { value: false, label: '🎬 Réalisateur inconnu' },
    { value: true, label: '🏆 Réalisateur reconnu' },
  ],
  country: [
    { value: '', label: 'Choisir un pays...' },
    { value: 'us', label: '🇺🇸 États-Unis' },
    { value: 'uk', label: '🇬🇧 Royaume-Uni' },
    { value: 'fr', label: '🇫🇷 France' },
    { value: 'jp', label: '🇯🇵 Japon' },
    { value: 'cn', label: '🇨🇳 Chine' },
    { value: 'au', label: '🇦🇺 Australie' },
    { value: 'ca', label: '🇨🇦 Canada' },
  ],
}

function updatePreview() {
  if (store.currentForm.budget && store.currentForm.genre) {
    preview.value = store.predictBoxOffice(store.currentForm)
  }
}

function submit() {
  store.addPrediction()
  submitted.value = true
  setTimeout(() => {
    submitted.value = false
    router.push('/predictions')
  }, 1500)
}

function formatM(val) {
  return val >= 1000 ? `${(val / 1000).toFixed(1)}Md$` : `${Math.round(val)}M$`
}

const tierColor = {
  Blockbuster: '#f59e0b',
  Hit: '#22c55e',
  Modéré: '#38bdf8',
  Risqué: '#ef4444'
}

const labelStyle = "display: block; color: rgba(255,255,255,0.6); font-size: 0.8rem; font-weight: 500; margin-bottom: 0.4rem; letter-spacing: 0.02em;"
</script>

<template>
  <div style="min-height: 100vh; background: #0a0a0f; padding: 2.5rem 1.5rem;">
    <div style="max-width: 1100px; margin: 0 auto;">

      <!-- Header -->
      <div style="margin-bottom: 2.5rem;">
        <p style="color: rgba(229,9,20,0.8); font-size: 0.75rem; font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase; margin-bottom: 0.5rem;">
          Moteur de prédiction
        </p>
        <h1 style="font-family: Georgia, serif; font-size: clamp(1.8rem, 4vw, 2.8rem); font-weight: 700; color: #fff; margin: 0; letter-spacing: -0.03em; line-height: 1.1;">
          Analyse de box office
        </h1>
        <p style="color: rgba(255,255,255,0.35); margin-top: 0.5rem; font-size: 0.95rem;">
          Sélectionne les paramètres du film pour obtenir une estimation mondiale
        </p>
      </div>

      <!-- Grid principal -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">

        <!-- Formulaire -->
        <div class="lg:col-span-2" style="background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.07); border-radius: 16px; padding: 2rem;">
          <h2 style="color: rgba(255,255,255,0.5); font-size: 0.7rem; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; margin-bottom: 1.75rem;">
            Informations du film
          </h2>

          <div style="display: flex; flex-direction: column; gap: 1.25rem;">

            <!-- Titre + Studio -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Titre du film</label>
                <select v-model="store.currentForm.title" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.title" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Studio de production</label>
                <select v-model="store.currentForm.studio" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.studio" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Genre + Pays -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Genre</label>
                <select v-model="store.currentForm.genre" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.genre" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Pays d'origine</label>
                <select v-model="store.currentForm.country" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.country" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Réalisateur + Acteur principal -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Réalisateur</label>
                <select v-model="store.currentForm.director" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.director" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Acteur principal</label>
                <select v-model="store.currentForm.lead_actor" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.lead_actor" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Budget + Marketing -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Budget production</label>
                <select v-model="store.currentForm.budget" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.budget" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Budget marketing</label>
                <select v-model="store.currentForm.marketing_budget" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.marketing_budget" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Durée + Saison -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Durée du film</label>
                <select v-model="store.currentForm.runtime" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.runtime" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Saison de sortie</label>
                <select v-model="store.currentForm.season" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.season" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Star Power + Réal connu -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label :style="labelStyle">Star Power</label>
                <select v-model="store.currentForm.star_power" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.star_power" :key="o.value" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
              <div>
                <label :style="labelStyle">Réalisateur</label>
                <select v-model="store.currentForm.director_known" @change="updatePreview" :style="selectStyle">
                  <option v-for="o in options.director_known" :key="String(o.value)" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
                </select>
              </div>
            </div>

            <!-- Sequel -->
            <div>
              <label :style="labelStyle">Type de film</label>
              <select v-model="store.currentForm.sequel" @change="updatePreview" :style="selectStyle">
                <option v-for="o in options.sequel" :key="String(o.value)" :value="o.value" style="background:#1a1a24;">{{ o.label }}</option>
              </select>
            </div>

            <!-- Submit -->
            <button @click="submit"
              :disabled="!store.currentForm.title || !store.currentForm.genre || !store.currentForm.budget"
              style="
                width: 100%; padding: 0.85rem; border-radius: 10px; border: none;
                background: #e50914; color: #fff;
                font-size: 0.95rem; font-weight: 700; letter-spacing: 0.02em;
                cursor: pointer; transition: all 0.2s;
                box-shadow: 0 0 24px rgba(229,9,20,0.3);
                margin-top: 0.25rem;
              "
              :style="(!store.currentForm.title || !store.currentForm.genre || !store.currentForm.budget)
                ? 'opacity: 0.35; cursor: not-allowed; box-shadow: none;' : ''"
            >
              {{ submitted ? '✅ Prédiction enregistrée !' : '🎯 Lancer la prédiction' }}
            </button>

          </div>
        </div>

        <!-- Colonne droite -->
        <div style="display: flex; flex-direction: column; gap: 1rem;">

          <!-- Preview live -->
          <div style="
            background: rgba(255,255,255,0.03);
            border: 1px solid rgba(255,255,255,0.07);
            border-radius: 16px; padding: 1.5rem;
            transition: opacity 0.3s;
          " :style="preview ? '' : 'opacity: 0.4'">

            <p style="color: rgba(255,255,255,0.35); font-size: 0.7rem; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; margin-bottom: 1.25rem;">
              Estimation en direct
            </p>

            <div v-if="preview">
              <div style="text-align: center; margin-bottom: 1.5rem;">
                <div style="font-family: Georgia, serif; font-size: 3rem; font-weight: 700; color: #fff; line-height: 1; letter-spacing: -0.03em;">
                  {{ formatM(preview.amount) }}
                </div>
                <div style="color: rgba(255,255,255,0.3); font-size: 0.75rem; margin-top: 0.4rem;">box office mondial estimé</div>
              </div>

              <div style="border-top: 1px solid rgba(255,255,255,0.07); padding-top: 1.25rem; display: flex; flex-direction: column; gap: 0.85rem;">
                <div style="display: flex; justify-content: space-between; align-items: center;">
                  <span style="color: rgba(255,255,255,0.4); font-size: 0.82rem;">Catégorie</span>
                  <span style="font-size: 0.75rem; font-weight: 700; padding: 3px 10px; border-radius: 99px;"
                    :style="`background: ${tierColor[preview.tier]}22; color: ${tierColor[preview.tier]}; border: 1px solid ${tierColor[preview.tier]}44`">
                    {{ preview.tier }}
                  </span>
                </div>
                <div style="display: flex; justify-content: space-between; align-items: center;">
                  <span style="color: rgba(255,255,255,0.4); font-size: 0.82rem;">ROI estimé</span>
                  <span style="font-weight: 700; font-size: 0.95rem;"
                    :style="preview.roi > 0 ? 'color: #22c55e' : 'color: #ef4444'">
                    {{ preview.roi > 0 ? '+' : '' }}{{ preview.roi }}%
                  </span>
                </div>
                <div>
                  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                    <span style="color: rgba(255,255,255,0.4); font-size: 0.82rem;">Confiance</span>
                    <span style="color: #f59e0b; font-weight: 700; font-size: 0.9rem;">{{ preview.confidence }}%</span>
                  </div>
                  <div style="background: rgba(255,255,255,0.07); border-radius: 99px; height: 4px; overflow: hidden;">
                    <div style="height: 100%; border-radius: 99px; background: #f59e0b; transition: width 0.4s ease;"
                      :style="`width: ${preview.confidence}%`"></div>
                  </div>
                </div>
              </div>
            </div>

            <div v-else style="text-align: center; padding: 2rem 0; color: rgba(255,255,255,0.2);">
              <div style="font-size: 2.5rem; margin-bottom: 0.75rem;">🎬</div>
              <p style="font-size: 0.82rem; line-height: 1.5;">Sélectionne le genre<br>et le budget pour voir l'estimation</p>
            </div>
          </div>

          <!-- Stats rapides -->
          <div v-if="store.predictions.length" style="
            background: rgba(229,9,20,0.08);
            border: 1px solid rgba(229,9,20,0.2);
            border-radius: 16px; padding: 1.5rem;
            display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;
          ">
            <div>
              <div style="color: rgba(255,255,255,0.35); font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 0.3rem;">Films</div>
              <div style="color: #fff; font-size: 2rem; font-weight: 700; font-family: Georgia, serif;">{{ store.predictions.length }}</div>
            </div>
            <div>
              <div style="color: rgba(255,255,255,0.35); font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 0.3rem;">ROI moyen</div>
              <div style="color: #22c55e; font-size: 2rem; font-weight: 700; font-family: Georgia, serif;">{{ store.avgRoi }}%</div>
            </div>
            <div v-if="store.topFilm" style="grid-column: 1 / -1; border-top: 1px solid rgba(255,255,255,0.07); padding-top: 1rem;">
              <div style="color: rgba(255,255,255,0.35); font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.1em; margin-bottom: 0.3rem;">Meilleur film</div>
              <div style="color: #f59e0b; font-size: 0.95rem; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">{{ store.topFilm.title }}</div>
            </div>
          </div>

        </div>
      </div>
    </div>
  </div>
</template>
