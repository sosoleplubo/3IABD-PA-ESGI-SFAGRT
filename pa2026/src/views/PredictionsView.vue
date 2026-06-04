<script setup>
import { useMovieStore } from '@/stores/movieStore'

const store = useMovieStore()

function formatM(val) {
  return val >= 1000 ? `${(val / 1000).toFixed(1)}Md$` : `${Math.round(val)}M$`
}

const tierColor = {
  Blockbuster: 'badge-warning',
  Hit: 'badge-success',
  Modéré: 'badge-info',
  Risqué: 'badge-error'
}

const tierBg = {
  Blockbuster: 'border-l-warning',
  Hit: 'border-l-success',
  Modéré: 'border-l-info',
  Risqué: 'border-l-error'
}
</script>

<template>
  <div class="min-h-screen bg-base-200 p-6">
    <div class="max-w-5xl mx-auto">

      <div class="mb-8 flex items-end justify-between">
        <div>
          <h1 class="text-4xl font-black tracking-tight">📊 Mes prédictions</h1>
          <p class="text-base-content/60 mt-1">{{ store.predictions.length }} film(s) analysé(s)</p>
        </div>
        <RouterLink to="/" class="btn btn-primary btn-sm">+ Nouvelle prédiction</RouterLink>
      </div>

      <!-- Stats globales -->
      <div v-if="store.predictions.length" class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <div class="stat bg-base-100 rounded-2xl shadow">
          <div class="stat-title">Films</div>
          <div class="stat-value text-primary">{{ store.predictions.length }}</div>
        </div>
        <div class="stat bg-base-100 rounded-2xl shadow">
          <div class="stat-title">ROI moyen</div>
          <div class="stat-value text-success text-2xl">{{ store.avgRoi }}%</div>
        </div>
        <div class="stat bg-base-100 rounded-2xl shadow col-span-2">
          <div class="stat-title">Meilleur film</div>
          <div class="stat-value text-warning text-xl truncate">{{ store.topFilm?.title || '—' }}</div>
          <div class="stat-desc">{{ store.topFilm ? formatM(store.topFilm.amount) : '' }}</div>
        </div>
      </div>

      <!-- Liste -->
      <div v-if="store.predictions.length" class="flex flex-col gap-4">
        <div v-for="p in store.predictions" :key="p.id"
          class="card bg-base-100 shadow border-l-4"
          :class="tierBg[p.tier]">
          <div class="card-body py-4">
            <div class="flex items-start justify-between flex-wrap gap-3">
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 flex-wrap">
                  <h3 class="font-black text-lg truncate">{{ p.title }}</h3>
                  <span class="badge" :class="tierColor[p.tier]">{{ p.tier }}</span>
                  <span v-if="p.sequel" class="badge badge-outline badge-sm">Franchise</span>
                </div>
                <div class="flex gap-4 mt-2 text-sm text-base-content/60 flex-wrap">
                  <span>🎬 {{ p.genre }}</span>
                  <span v-if="p.season">📅 {{ p.season }}</span>
                  <span>💰 Budget: {{ p.budget }}M$</span>
                  <span v-if="p.director_known">🎥 Réal. connu</span>
                  <span>⭐ Star power: {{ p.star_power }}/5</span>
                </div>
              </div>

              <div class="text-right shrink-0">
                <div class="text-3xl font-black text-primary">{{ formatM(p.amount) }}</div>
                <div class="text-sm mt-1">
                  <span :class="p.roi > 0 ? 'text-success' : 'text-error'" class="font-bold">
                    ROI {{ p.roi > 0 ? '+' : '' }}{{ p.roi }}%
                  </span>
                  <span class="text-base-content/40 ml-2">· {{ p.confidence }}% confiance</span>
                </div>
                <div class="text-xs text-base-content/30 mt-1">{{ p.date }}</div>
              </div>
            </div>

            <div class="flex justify-end mt-2">
              <button @click="store.removePrediction(p.id)" class="btn btn-ghost btn-xs text-error">
                🗑 Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty state -->
      <div v-else class="text-center py-20">
        <div class="text-6xl mb-4">🎬</div>
        <h2 class="text-2xl font-bold mb-2">Aucune prédiction</h2>
        <p class="text-base-content/50 mb-6">Lance ta première analyse de film</p>
        <RouterLink to="/" class="btn btn-primary">Commencer</RouterLink>
      </div>

    </div>
  </div>
</template>
