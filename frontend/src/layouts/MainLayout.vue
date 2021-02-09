<template>
  <q-layout view="hHh lpr fFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn icon="home" aria-label="Accueil" to="/"
          flat dense round data-test-id="menu-home-link"/>
        <q-separator dark vertical inset class="q-mr-sm"/>
        <q-btn icon="leaderboard" label="Évaluations" to="/evaluations/"
          flat dense data-test-id="menu-evaluations-link"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn icon="person" label="Élèves" to="/eleves/"
          flat dense data-test-id="menu-eleves-link"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn icon="timeline" label="Rapports" to="/rapports/"
          flat dense data-test-id="menu-report-link"/>
        <q-separator dark vertical inset class="q-mx-sm"/>
        <q-btn icon="school" label="Socle" to="/socle/"
          flat dense data-test-id="menu-socle-link"/>
        <q-space/>
        <q-btn icon="support" label="À propos" to="/support/"
          flat dense data-test-id="menu-support-link"/>
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
