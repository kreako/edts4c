<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row q-ml-md q-mt-md">
        <q-breadcrumbs>
          <q-breadcrumbs-el icon="home" to="/socle/" />
          <q-breadcrumbs-el :label="`${domain.rank}. ${domain.title}`"
            :to="`/socle/domain/${$route.params.domainId}`" />
        </q-breadcrumbs>
      </div>
      <div class="row q-ml-md q-mt-xl">
        <div class="col-auto">
          <q-list bordered>
            <draggable v-model="components" ghost-class="ghost" handle=".handle">
              <q-item v-for="component in components" :key="component.id" class="show-children-on-hover">
                <q-item-section avatar top>
                  <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                    {{ component.rank }}
                  </q-avatar>
                </q-item-section>
                <q-item-section v-if="editId === component.id">
                  <q-input v-model="editTitle" @keydown.enter="save" filled dense/>
                  <q-btn color="primary" text-color="white" icon="save" label="Sauvegarder" size="sm" @click="save"/>
                </q-item-section>
                <q-item-section v-else>
                  <div class="row">
                    <router-link :to="`/socle/domain/${$route.params.domainId}/component/${component.id}`" class="clean-link">
                      {{ component.title }}
                    </router-link>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="edit" size="xs"
                      @click="edit(component.id)"/>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="delete" size="xs"
                      @click="remove(component.id)"/>
                    <q-btn flat round class="q-ml-md show-on-hover"
                      text-color="grey-9" icon="drive_file_move" size="xs"
                      @click="showMoveSelector(component.id)"/>
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
            <q-btn :to="`/socle/domain/${previousDomainId}`"
              :disable="!previousDomainId"
              icon="navigate_before"/>
            <q-btn :to="`/socle/domain/${nextDomainId}`"
              :disable="!nextDomainId"
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
import MoveComponent from '../components/MoveComponent.vue'
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
      nextDomainId: state => state.socle.nextDomainId,
      previousDomainId: state => state.socle.previousDomainId,
      domains: state => state.socle.domains
    }),
    components: {
      get () {
        return this.$store.state.socle.components
      },
      set (value) {
        this.$store.dispatch('socle/setComponents', value)
      }
    }
  },
  methods: {
    edit: function (componentId) {
      this.editId = componentId
      this.editTitle = this.$store.getters['socle/componentFromId'](componentId).title
    },
    save: function () {
      this.$store.dispatch('socle/setComponentTitle', { componentId: this.editId, title: this.editTitle })
      this.editId = -1
    },
    showMoveSelector: function (componentId) {
      this.$q.dialog({
        component: MoveComponent,
        parent: this,
        componentId: componentId
      }).onOk((data) => {
        this.$store.dispatch('socle/moveComponent', { componentId: componentId, domainId: data.domainId })
        this.$router.push(`/socle/domain/${data.domainId}`)
      })
    },
    remove: function (componentId) {
      this.$q.dialog({
        title: 'Confirmation',
        message: 'Êtes-vous sur de vouloir supprimer cette composante avec toutes ses compétences ? (Attention, ce sera définitif !!!)',
        cancel: true,
        persistent: true
      }).onOk(() => {
        this.$store.dispatch('socle/removeComponent', componentId)
      })
    },
    add: function () {
      this.$q.dialog({
        title: 'Nouvelle composante',
        message: 'Entrez son titre :',
        prompt: {
          model: '',
          type: 'text' // optional
        },
        cancel: true,
        persistent: true
      }).onOk(data => {
        this.$store.dispatch('socle/addComponent', { domainId: this.domain.id, title: data })
      })
    }
  },
  watch: {
    '$route.params.domainId': function (domainId) {
      this.$store.dispatch('socle/loadComponents', this.$route.params.domainId)
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadComponents', this.$route.params.domainId)
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
