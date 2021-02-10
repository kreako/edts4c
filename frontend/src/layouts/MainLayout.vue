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

    <q-dialog v-model="inError" persistent>
      <q-card>
        <q-card-section class="bg-negative">
          <div class="text-h6 text-white">Erreur</div>
        </q-card-section>

        <q-card-section class="">
          <div class="text-h6">
            Une erreur est survenue et j'en suis bien désolé.
          </div>
          <div class="text-h6">
            Merci de copier-coller tout ce qui s'affiche en dessous de ce message
            et de me l'envoyer par mail !
          </div>
          <div class="pre-wrap">{{ message }}</div>
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat label="OK, C'est bon, c'est envoyé !" color="negative" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-layout>
</template>

<script>
import { mapState } from 'vuex'
export default {
  name: 'MainLayout',
  data () {
    return {
    }
  },
  computed: {
    ...mapState({
      message: state => state.error.message
    }),
    inError: {
      get () {
        return this.$store.state.error.inError
      },
      set (value) {
        this.$store.commit('error/setInError', value)
      }
    }
  }
}
</script>

<style>
.pre-wrap {
  white-space: pre-wrap;
}
</style>
