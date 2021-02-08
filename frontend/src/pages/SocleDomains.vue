<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row q-ml-md q-mt-md">
        <q-breadcrumbs>
          <q-breadcrumbs-el icon="home" to="/socle/" />
        </q-breadcrumbs>
      </div>
      <div class="row q-ml-md q-mt-xl">
        <div class="col-auto">
          <q-list bordered>
            <draggable v-model="domains" ghost-class="ghost" handle=".handle">
              <q-item v-for="(domain, index) in domains" :key="domain.id" class="show-children-on-hover">
                <q-item-section avatar>
                  <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                    {{ index + 1 }}
                  </q-avatar>
                </q-item-section>
                <q-item-section v-if="editId === domain.id">
                  <q-input v-model="editTitle" @keydown.enter="save" filled dense/>
                  <q-btn color="primary" text-color="white" icon="save" label="Sauvegarder" size="sm" @click="save"/>
                </q-item-section>
                <q-item-section v-else>
                  <div class="row">
                    <router-link :to="`/socle/domain/${domain.id}`" class="clean-link">
                      {{ domain.title }}
                    </router-link>
                    <q-icon class="q-ml-md show-on-hover"
                      color="grey-9" name="edit" size="xs" @click="edit(domain.id)"/>
                    <q-icon class="q-ml-md show-on-hover"
                      color="grey-9" name="delete" size="xs" @click="remove(domain.id)"/>
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
      loading: state => state.socle.loading
    }),
    domains: {
      get () {
        return this.$store.state.socle.domains
      },
      set (value) {
        this.$store.dispatch('socle/setDomains', value)
      }
    }
  },
  methods: {
    edit: function (domainId) {
      this.editId = domainId
      this.editTitle = this.$store.getters['socle/domainFromId'](domainId).title
    },
    save: function () {
      this.$store.dispatch('socle/setDomainTitle', { domainId: this.editId, title: this.editTitle })
      this.editId = -1
    },
    remove: function (domainId) {
      this.$q.dialog({
        title: 'Confirmation',
        message: 'Êtes-vous sur de vouloir supprimer ce domaine avec tous ses composantes/compétences ? (Attention, ce sera définitif !!!)',
        cancel: true,
        persistent: true
      }).onOk(() => {
        this.$store.dispatch('socle/removeDomain', domainId)
      })
    },
    add: function () {
      this.$q.dialog({
        title: 'Nouveau domaine',
        message: 'Entrez son titre :',
        prompt: {
          model: '',
          type: 'text' // optional
        },
        cancel: true,
        persistent: true
      }).onOk(data => {
        this.$store.dispatch('socle/addDomain', data)
      })
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadDomains')
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
