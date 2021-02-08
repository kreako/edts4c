<template>
  <q-dialog ref="dialog" @hide="onDialogHide">
    <q-card class="q-dialog-plugin q-pa-md">
      <q-list bordered padding>
        <q-item>
          <q-item-section>
            <q-item-label overline>
              Déplacer cette composante vers...
            </q-item-label>
          </q-item-section>
        </q-item>
        <div v-for="domain in domains" :key="domain.id">
          <q-item :clickable="domain.id != $route.params.domainId"
            :disable="domain.id == $route.params.domainId"
            @click="onDomainClick(domain.id, componentId)">
            <q-item-section avatar>
              <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                {{ domain.rank }}
              </q-avatar>
            </q-item-section>
            <q-item-section>
              {{ domain.title }}
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
    componentId: Number
  },
  computed: {
    ...mapState({
      domains: state => state.socle.domains
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
    onDomainClick (domainId, componentId) {
      this.$emit('ok', { domainId: domainId, componentId: componentId })
      this.hide()
    },
    onCancelClick () {
      this.hide()
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadDomains')
  }
}
</script>
