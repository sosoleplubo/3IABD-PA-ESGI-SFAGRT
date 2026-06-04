<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { useMovieStore } from '@/stores/movieStore'
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const store = useMovieStore()
const route = useRoute()
const menuOpen = ref(false)
</script>

<template>
  <div data-theme="night" class="min-h-screen" style="background: #0a0a0f;">

    <nav style="
      position: sticky; top: 0; z-index: 50;
      background: rgba(10, 10, 15, 0.85);
      backdrop-filter: blur(20px) saturate(180%);
      -webkit-backdrop-filter: blur(20px) saturate(180%);
      border-bottom: 1px solid rgba(255,255,255,0.06);
    ">
      <div class="max-w-6xl mx-auto px-6" style="height: 64px; display: flex; align-items: center; justify-content: space-between;">

        <!-- Logo -->
        <RouterLink to="/" class="no-underline flex items-center gap-2 shrink-0">
          <div style="
            width: 32px; height: 32px; border-radius: 8px;
            background: linear-gradient(135deg, #e50914 0%, #ff6b35 100%);
            display: flex; align-items: center; justify-content: center;
            font-size: 16px;
            box-shadow: 0 0 20px rgba(229, 9, 20, 0.4);
          ">🎬</div>
          <span style="font-family: Georgia, serif; font-size: 1.2rem; font-weight: 700; color: #fff; letter-spacing: -0.02em;">
            Cine<span style="color: #e50914;">Predict</span>
          </span>
        </RouterLink>

        <!-- Desktop links (caché sur mobile) -->
        <div class="hidden md:flex items-center gap-2">
          <RouterLink to="/" class="no-underline px-4 py-2 rounded-lg text-sm font-medium transition-all"
            :style="route.path === '/' ? 'color:#fff; background:rgba(255,255,255,0.08)' : 'color:rgba(255,255,255,0.55)'">
            Analyser
          </RouterLink>
          <RouterLink to="/predictions" class="no-underline px-4 py-2 rounded-lg text-sm font-medium flex items-center gap-2 transition-all"
            :style="route.path === '/predictions' ? 'color:#fff; background:rgba(255,255,255,0.08)' : 'color:rgba(255,255,255,0.55)'">
            Prédictions
            <span v-if="store.predictions.length" style="
              background: #e50914; color: #fff;
              font-size: 0.7rem; font-weight: 700;
              padding: 1px 7px; border-radius: 99px; line-height: 1.6;
            ">{{ store.predictions.length }}</span>
          </RouterLink>
          <RouterLink to="/" class="no-underline ml-2 px-4 py-2 rounded-lg text-sm font-semibold transition-all"
            style="background: #e50914; color: #fff; box-shadow: 0 0 16px rgba(229,9,20,0.35);">
            + Nouveau
          </RouterLink>
        </div>

        <!-- Burger menu (visible sur mobile) -->
        <button class="md:hidden btn btn-ghost btn-sm px-2" @click="menuOpen = !menuOpen"
          style="color: rgba(255,255,255,0.7);">
          <svg v-if="!menuOpen" xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
          </svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <!-- Menu mobile déroulant -->
      <div v-if="menuOpen" class="md:hidden px-6 pb-4 flex flex-col gap-2"
        style="border-top: 1px solid rgba(255,255,255,0.06);">
        <RouterLink to="/" @click="menuOpen = false" class="no-underline px-4 py-3 rounded-lg text-sm font-medium transition-all"
          :style="route.path === '/' ? 'color:#fff; background:rgba(255,255,255,0.08)' : 'color:rgba(255,255,255,0.55)'">
          Analyser
        </RouterLink>
        <RouterLink to="/predictions" @click="menuOpen = false" class="no-underline px-4 py-3 rounded-lg text-sm font-medium flex items-center gap-2 transition-all"
          :style="route.path === '/predictions' ? 'color:#fff; background:rgba(255,255,255,0.08)' : 'color:rgba(255,255,255,0.55)'">
          Prédictions
          <span v-if="store.predictions.length" style="
            background: #e50914; color: #fff;
            font-size: 0.7rem; font-weight: 700;
            padding: 1px 7px; border-radius: 99px; line-height: 1.6;
          ">{{ store.predictions.length }}</span>
        </RouterLink>
        <RouterLink to="/" @click="menuOpen = false" class="no-underline px-4 py-3 rounded-lg text-sm font-semibold text-center"
          style="background: #e50914; color: #fff;">
          + Nouveau
        </RouterLink>
      </div>
    </nav>

    <RouterView />

  </div>
</template>
