<template>
  <q-card>
    <q-card-section>
        <q-btn :to="`/evaluation/${cycle}/start`"
          :label="`Cycle ${cycle}`"
          :color="cycleColor(cycle)"/>
        <q-list>
          <q-item>
            <q-item-section>
              <q-item-label class="nowrap">
                {{ stats.nb_eleves }} élèves
              </q-item-label>
              <q-item-label class="nowrap">
                {{ stats.empty }} non évaluées
              </q-item-label>
              <q-item-label class="nowrap">
                {{ stats.in_progress }} en cours
              </q-item-label>
              <q-item-label class="nowrap">
                {{ stats.not_acquired }} non acquis
              </q-item-label>
              <q-item-label class="nowrap">
                {{ stats.acquired }} acquis
              </q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
    </q-card-section>
  </q-card>
</template>

<script>
import { CYCLE_COLORS } from '../pages/cycle-color.js'

export default {
  props: {
    stats: Object,
    cycle: String
  },
  data () {
    return {}
  },
  methods: {
    cycleColor: function (cycle) {
      return CYCLE_COLORS[cycle]
    }
  }
}
</script>

<style>
.nowrap {
  white-space: nowrap;
}
</style>
