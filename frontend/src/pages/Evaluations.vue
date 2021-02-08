<template>
  <q-page padding>
    <!-- content -->
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row">
        <div class="col">
        </div>
        <div class="col-auto">
          <div class="row">
            <div class="col">
              <q-card>
                <q-card-section>
                  <div v-if="!evaluationDateInEdit && evaluationDateSaved">
                    Date de l'évaluation : {{ evaluationDate }}
                    <q-btn icon="edit" flat size="xs" @click="editEvaluationDate"/>
                  </div>
                  <div v-if="!evaluationDateInEdit && !evaluationDateSaved">
                    <q-spinner-gears color="primary"/>
                  </div>
                  <div v-if="evaluationDateInEdit">
                    <div class="row items-center q-gutter-md">
                      <div>
                        Date de l'évaluation :
                      </div>
                      <div>
                        <q-input v-model="evaluationDateEdit" mask="####-##-##" fill-mask @keydown.enter="saveEvaluationDate" filled>
                          <template v-slot:append>
                            <q-icon name="event" class="cursor-pointer">
                              <q-popup-proxy transition-show="scale" transition-hide="scale">
                                <q-date v-model="evaluationDateEdit" mask="YYYY-MM-DD">
                                  <div class="row items-center justify-end">
                                    <q-btn v-close-popup label="OK" color="primary" flat />
                                  </div>
                                </q-date>
                              </q-popup-proxy>
                            </q-icon>
                          </template>
                        </q-input>
                      </div>
                      <div>
                        <q-btn color="primary" text-color="white" icon="save" label="Sauvegarder" size="sm" @click="saveEvaluationDate"/>
                      </div>
                    </div>
                  </div>
                </q-card-section>
              </q-card>
            </div>
          </div>
          <div class="row q-gutter-lg q-mt-xs">
            <div class="col">
              <CycleCard cycle="C1" :stats="stats.c1"/>
            </div>
            <div class="col">
              <CycleCard cycle="C2" :stats="stats.c2"/>
            </div>
          </div>
          <div class="row q-gutter-lg q-mt-xs">
            <div class="col">
              <CycleCard cycle="C3" :stats="stats.c3"/>
            </div>
            <div class="col">
              <CycleCard cycle="C4" :stats="stats.c4"/>
            </div>
          </div>
        </div>
        <div class="col">
        </div>
      </div>
    </div>
  </q-page>
</template>

<script>
import { mapState } from 'vuex'
import { CYCLE_COLORS } from './cycle-color.js'
import CycleCard from '../components/CycleCard.vue'

export default {
  components: { CycleCard },
  data () {
    return {
      evaluationDateInEdit: false,
      evaluationDateEdit: ''
    }
  },
  computed: {
    ...mapState({
      loading: state => state.evaluations.loading,
      evaluationDate: state => state.evaluations.evaluationDate,
      stats: state => state.evaluations.stats
    }),
    evaluationDateSaved: function () {
      return this.evaluationDateEdit === '' || this.evaluationDateEdit === this.evaluationDate
    }
  },
  methods: {
    cycleColor: function (cycle) {
      return CYCLE_COLORS[`C${cycle}`]
    },
    editEvaluationDate: function () {
      this.evaluationDateEdit = this.evaluationDate
      this.evaluationDateInEdit = true
    },
    saveEvaluationDate: async function () {
      this.evaluationDateInEdit = false
      await this.$store.dispatch('evaluations/setEvaluationDate', this.evaluationDateEdit)
    }
  },
  mounted: function () {
    this.$store.dispatch('evaluations/load')
  }
}
</script>
