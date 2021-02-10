<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row q-ml-md q-mt-md">
        <q-breadcrumbs>
          <q-breadcrumbs-el icon="home" to="/socle/" />
          <q-breadcrumbs-el
            :label="`${domain.rank}. ${domain.title.slice(0, 30)}`"
            :to="`/socle/domain/${$route.params.domainId}`" />
          <q-breadcrumbs-el
            :label="`${component.rank}. ${component.title}`"
            :to="`/socle/domain/${$route.params.domainId}/component/${$route.params.componentId}`" />
        </q-breadcrumbs>
      </div>
      <div class="row q-ml-md q-mt-xl">
        <div class="col-auto">
          <q-list bordered>
            <draggable v-model="competencies" ghost-class="ghost" handle=".handle">
              <q-item v-for="competency in competencies" :key="competency.id" class="show-children-on-hover">
                <q-item-section avatar top>
                  <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                    {{ competency.rank }}
                  </q-avatar>
                </q-item-section>
                <q-item-section v-if="editId === competency.id">
                  <q-input v-model="editTitle" @keydown.enter="save" filled dense/>
                  <q-btn color="primary" text-color="white" icon="save" label="Sauvegarder" size="sm" @click="save"/>
                </q-item-section>
                <q-item-section v-else>
                  <div class="row">
                    <router-link :to="`/socle/domain/${$route.params.domainId}/component/${component.id}/competency/${competency.id}`" class="clean-link">
                      {{ competency.title }}
                    </router-link>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="edit" size="xs"
                      @click="edit(competency.id)"/>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="delete" size="xs"
                      @click="remove(competency.id)"/>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="drive_file_move" size="xs"
                      @click="showMoveSelector(competency.id)"/>
                    <q-icon class="q-ml-md q-mr-xl cursor-move handle show-on-hover"
                      color="grey-9" name="drag_indicator" size="xs"/>
                  </div>
                </q-item-section>
              </q-item>
            </draggable>
            <q-item class="q-mt-md">
              <q-item-section class="q-px-md">
                <q-btn icon="add"
                  label="Ajouter" size="sm" @click="add"/>
              </q-item-section>
            </q-item>
          </q-list>
          <div class="row justify-center q-mt-xl">
            <q-btn :to="`/socle/domain/${domain.id}/component/${previousComponentId}`"
              :disable="!previousComponentId"
              icon="navigate_before"/>
            <q-btn :to="`/socle/domain/${domain.id}/component/${nextComponentId}`"
              :disable="!nextComponentId"
              icon="navigate_next"/>
          </div>
        </div>
        <div class="col">
        </div>
      </div>
    </div>
  </q-page>
</template>

<script>
import draggable from 'vuedraggable'
import { mapState } from 'vuex'
import MoveCompetency from '../components/MoveCompetency.vue'
export default {
  components: {
    draggable
  },
  data () {
    return {
      editId: -1,
      editTitle: ''
    }
  },
  computed: {
    ...mapState({
      loading: state => state.socle.loading,
      domain: state => state.socle.domain,
      component: state => state.socle.component,
      nextComponentId: state => state.socle.nextComponentId,
      previousComponentId: state => state.socle.previousComponentId
    }),
    competencies: {
      get () {
        return this.$store.state.socle.competencies
      },
      set (value) {
        this.$store.dispatch('socle/setCompetencies', value)
      }
    }
  },
  methods: {
    edit: function (competencyId) {
      this.editId = competencyId
      this.editTitle = this.$store.getters['socle/competencyFromId'](competencyId).title
    },
    save: function () {
      this.$store.dispatch('socle/setCompetencyTitle', { competencyId: this.editId, title: this.editTitle })
      this.editId = -1
    },
    showMoveSelector: function (competencyId) {
      this.$q.dialog({
        component: MoveCompetency,
        parent: this,
        competencyId: competencyId
      }).onOk((data) => {
        this.$store.dispatch('socle/moveCompetency', { competencyId: competencyId, componentId: data.componentId, domainId: data.domainId })
        this.$router.push(`/socle/domain/${data.domainId}/component/${data.componentId}`)
      })
    },
    remove: function (competencyId) {
      this.$q.dialog({
        title: 'Confirmation',
        message: 'Êtes-vous sur de vouloir supprimer cette compétence ? (Attention, ce sera définitif !!!)',
        cancel: true,
        persistent: true
      }).onOk(() => {
        this.$store.dispatch('socle/removeCompetency', competencyId)
      })
    },
    add: function () {
      this.$q.dialog({
        title: 'Nouvelle compétence',
        message: 'Entrez son titre :',
        prompt: {
          model: '',
          type: 'text' // optional
        },
        cancel: true,
        persistent: true
      }).onOk(data => {
        this.$store.dispatch('socle/addCompetency', { componentId: this.component.id, title: data })
      })
    }
  },
  watch: {
    '$route.params.domainId': function (domainId) {
      this.$store.dispatch('socle/loadCompetencies', this.$route.params.componentId)
    },
    '$route.params.componentId': function (componentId) {
      this.$store.dispatch('socle/loadCompetencies', this.$route.params.componentId)
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadCompetencies', this.$route.params.componentId)
  }
}
</script>
<style lang="scss">
.cursor-move {
  cursor: move;
}
.ghost {
  background-color: $blue-3;
}
.clean-link {
  color: inherit;
  text-decoration: inherit;
}
.show-children-on-hover .show-on-hover {
  visibility: hidden;
}
.show-children-on-hover:hover .show-on-hover {
  visibility: visible;
}
.show-children-on-hover .hide-on-hover {
  visibility: visible;
}
.show-children-on-hover:hover .hide-on-hover {
  visibility: hidden;
}
</style>
