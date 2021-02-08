<template>
  <q-page padding>
    <div class="row">
      <div class="col">
      </div>
      <div class="col-auto">
        <q-card>
          <q-card-section>
            <div class="text-h6">
              Rapport détaillé
            </div>
          </q-card-section>
          <q-card-section>
            <q-list>
              <q-item v-for="cycle in ['C1', 'C2', 'C3', 'C4']" :key="cycle"
                clickable v-ripple
                :to="`/print/rapport/full/${cycle}`">
                <q-item-section :class="`text-${cycleColor(cycle)}`">
                  Cycle {{ cycle }}
                </q-item-section>
              </q-item>
              <q-item clickable v-ripple to="/print/rapport/full/socle">
                <q-item-section>
                  Socle
                </q-item-section>
              </q-item>
            </q-list>
          </q-card-section>
        </q-card>
      </div>
      <div class="col-auto q-ml-md">
        <q-card>
          <q-card-section>
            <div class="text-h6">
              Rapport résumé
            </div>
          </q-card-section>
          <q-card-section>
            <q-list>
              <q-item v-for="cycle in ['C1', 'C2', 'C3', 'C4']" :key="cycle"
                clickable v-ripple
                :to="`/print/rapport/summary/${cycle}`">
                <q-item-section :class="`text-${cycleColor(cycle)}`">
                  Cycle {{ cycle }}
                </q-item-section>
              </q-item>
            </q-list>
          </q-card-section>
        </q-card>
      </div>
      <div class="col">
      </div>
    </div>
  </q-page>
</template>

<script>
import { CYCLE_COLORS } from './cycle-color.js'
export default {
  methods: {
    cycleColor: function (cycle) {
      return CYCLE_COLORS[cycle]
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
