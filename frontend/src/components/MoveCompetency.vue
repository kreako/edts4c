<template>
  <q-dialog ref="dialog" @hide="onDialogHide">
    <q-card class="q-dialog-plugin q-pa-md">
      <q-list bordered padding>
        <q-item>
          <q-item-section>
            <q-item-label overline>
              Déplacer cette compétence vers...
            </q-item-label>
          </q-item-section>
        </q-item>
        <div v-for="domain in domainsWithComponents" :key="domain.id">
          <q-item-label header>
            {{ domain.rank }} {{ domain.title }}
          </q-item-label>
          <q-item v-for="component in domain.components" :key="component.id"
            :clickable="(domain.id != $route.params.domainId) || (component.id != $route.params.componentId)"
            :disable="(domain.id == $route.params.domainId) && (component.id == $route.params.componentId)"
            @click="onComponentClick(domain.id, component.id)">
            <q-item-section avatar>
              <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                {{ component.rank }}
              </q-avatar>
            </q-item-section>
            <q-item-section>
              {{ component.title }}
            </q-item-section>
          </q-item>
        </div>
      </q-list>
      <q-card-actions align="right">
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script>
import { mapState } from 'vuex'
export default {
  data () {
    return {}
  },
  props: {
    competencyId: Number
  },
  computed: {
    ...mapState({
      domainsWithComponents: state => state.socle.domainsWithComponents
    })
  },
  methods: {
    show () {
      this.$refs.dialog.show()
    },
    hide () {
      this.$refs.dialog.hide()
    },
    onDialogHide () {
      this.$emit('hide')
    },
    onComponentClick (domainId, componentId) {
      this.$emit('ok', { domainId: domainId, componentId: componentId })
      this.hide()
    },
    onCancelClick () {
      this.hide()
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadDomainsWithComponents')
  }
}
</script>
