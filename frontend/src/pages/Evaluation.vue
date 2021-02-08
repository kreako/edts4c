<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div v-if="$route.params.componentId === 'start'">
        <div class="row">
          <div class="col">
          </div>
          <div class="col-auto">
            <q-card class="q-mb-md">
              <q-card-section>
                <div class="text-h6">
                  Évaluation
                </div>
                <div :class="`text-subtitle2 text-${cycleColor($route.params.cycle)}`">
                  Cycle {{ $route.params.cycle }}
                </div>
              </q-card-section>
              <q-card-section class="q-mb-md">
                <div class="column">
                  <q-btn label="Démarrer l'évaluation" color="primary"
                  :to="`/evaluation/${$route.params.cycle}/1`"/>
                  <div class="text-caption text-grey-5">
                    (Ou sélectionner une des composantes ci-dessous.)
                  </div>
                </div>
              </q-card-section>
            </q-card>
          </div>
          <div class="col">
          </div>
        </div>
        <EvalToc :toc="toc"/>
      </div>
      <div v-else-if="$route.params.componentId === 'end'">
        <EvalEnd @save="save" :comment="comment"/>
      </div>
      <div v-else>
        <EvalDetail @save="save" :detail="detail"/>
      </div>
    </div>
  </q-page>
</template>

<script>
import { mapState } from 'vuex'
import { CYCLE_COLORS } from './cycle-color.js'
import EvalToc from '../components/EvalToc.vue'
import EvalDetail from '../components/EvalDetail.vue'
import EvalEnd from '../components/EvalEnd.vue'

export default {
  components: {
    EvalToc, EvalDetail, EvalEnd
  },
  computed: {
    ...mapState({
      loading: state => state.evaluation.loading,
      toc: state => state.evaluation.toc,
      detail: state => state.evaluation.detail,
      comment: state => state.evaluation.comment
    })
  },
  methods: {
    cycleColor: function (cycle) {
      return CYCLE_COLORS[cycle]
    },
    save: function (statuses) {
      if (this.$route.params.componentId === 'end') {
        // Go back to start
        this.$router.push(`/evaluation/${this.$route.params.cycle}/start`)
      } else {
        if (this.detail.next_component_id) {
          // Now go to the next component
          this.$router.push(`/evaluation/${this.$route.params.cycle}/${this.detail.next_component_id}`)
        } else {
          // Or go to the end with general comment if no more component
          this.$router.push(`/evaluation/${this.$route.params.cycle}/end`)
        }
      }
    }
  },
  watch: {
    '$route.params.componentId': function (componentId) {
      if (this.$route.params.componentId === 'start') {
        this.$store.dispatch('evaluation/loadToc', this.$route.params.cycle)
      } else if (this.$route.params.componentId === 'end') {
        this.$store.dispatch('evaluation/loadComment', this.$route.params.cycle)
      } else {
        this.$store.dispatch('evaluation/loadDetail',
          {
            cycle: this.$route.params.cycle,
            componentId: componentId
          })
      }
    }
  },
  mounted: function () {
    if (this.$route.params.componentId === 'start') {
      this.$store.dispatch('evaluation/loadToc', this.$route.params.cycle)
    } else if (this.$route.params.componentId === 'end') {
      this.$store.dispatch('evaluation/loadComment', this.$route.params.cycle)
    } else {
      this.$store.dispatch('evaluation/loadDetail',
        {
          cycle: this.$route.params.cycle,
          componentId: this.$route.params.componentId
        })
    }
  }
}
</script>
<style>
.clean-link {
  color: inherit;
  text-decoration: inherit;
}
</style>
