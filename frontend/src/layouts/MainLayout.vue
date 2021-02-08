<template>
  <q-layout view="hHh lpr fFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn flat dense round data-cy="menu-home-link" icon="home" aria-label="Accueil" to="/"/>
        <q-separator dark vertical inset class="q-mr-sm"/>
        <q-btn flat dense data-cy="menu-evaluations-link" icon="leaderboard" label="Évaluations" aria-label="Évaluation" to="/evaluations"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn flat dense data-cy="menu-eleves-link" icon="person" label="Élèves" aria-label="Élèves" to="/eleves"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn flat dense data-cy="menu-report-link" icon="timeline" label="Rapports" aria-label="Rapport" to="/rapports"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn flat dense data-cy="menu-socle-link" icon="school" label="Socle" aria-label="Socle" to="/socle/"/>
        <q-space/>
        <q-btn flat dense data-cy="menu-support-link" icon="support" label="À propos" aria-label="À propos" to="/support"/>

      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>

export default {
  name: 'MainLayout',
  data () {
    return {
    }
  }
}
</script>
