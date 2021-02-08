<template>
  <div>
    <div v-for="domain in socle" :key="domain.id">
      <div class="text-h6 q-mt-md">
        {{ domain.rank }}. {{ domain.title }}
      </div>
      <div v-for="component in domain.components" :key="component.id">
        <div class="text-subtitle2 q-mt-md">
          {{ component.rank }}. {{ component.title }}
        </div>
        <div v-for="competency in component.competencies" :key="competency.id">
          <q-card class="q-mt-md">
            <q-card-section>
              {{ competency.rank }}. {{ competency.title }}
            </q-card-section>
            <q-card-section horizontal>
              <q-card-section class="col-6">
                <q-chip :color="cycleColor('C1')" text-color="white" size="sm">
                  Cycle C1
                </q-chip>
                <div class="pre-wrap">{{ competency.c1 }}</div>
              </q-card-section>
              <q-separator vertical/>
              <q-card-section class="col-6">
                <q-chip :color="cycleColor('C2')" text-color="white" size="sm">
                  Cycle C2
                </q-chip>
                <div class="pre-wrap">{{ competency.c2 }}</div>
              </q-card-section>
            </q-card-section>
            <q-separator/>
            <q-card-section horizontal>
              <q-card-section class="col-6">
                <q-chip :color="cycleColor('C3')" text-color="white" size="sm">
                  Cycle C3
                </q-chip>
                <div class="pre-wrap">{{ competency.c3 }}</div>
              </q-card-section>
              <q-separator vertical/>
              <q-card-section class="col-6">
                <q-chip :color="cycleColor('C4')" text-color="white" size="sm">
                  Cycle C4
                </q-chip>
                <div class="pre-wrap">{{ competency.c4 }}</div>
              </q-card-section>
            </q-card-section>
          </q-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { CYCLE_COLORS } from '../pages/cycle-color.js'

export default {
  props: {
    socle: Array
  },
  methods: {
    cycleColor: function (cycle) {
      return CYCLE_COLORS[cycle]
    }
  },
  data () {
    return {}
  }
}
</script>
<style>
.pre-wrap {
  white-space: pre-wrap;
}
</style>
