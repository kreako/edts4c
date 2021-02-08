<template>
  <div>
    <div class="text-h6">
      {{ detail.domain.rank }}. {{ detail.domain.title }}
    </div>
    <div class="text-subtitle2">
      {{ detail.component.rank }}. {{ detail.component.title }}
    </div>
    <div class="row">
      <div class="col">
      </div>
      <div class="col-auto">
        <div v-for="competency in detail.competencies" :key="competency.id">
          <q-card class="q-mt-md">
            <q-item>
              <q-item-section avatar>
                <q-chip>
                  {{ competency.rank }}
                </q-chip>
              </q-item-section>
              <q-item-section>
                <q-item-label>
                  <q-expansion-item expand-separator :label="competency.title">
                    <q-card>
                      <q-card-section class="pre-wrap">{{ competency.detail }}</q-card-section>
                    </q-card>
                  </q-expansion-item>
                </q-item-label>
              </q-item-section>
            </q-item>
            <q-separator/>
            <q-card-section>
              <q-markup-table>
                <tbody>
                  <tr v-for="evaluation in competency.evaluations" :key="evaluation.id">
                    <td>
                      {{ evaluation.eleve_firstname }} {{ evaluation.eleve_lastname }}
                    </td>
                    <td>
                      <q-btn v-if="evaluation.status === 'Empty'"
                        :color="statusColor('Empty')"
                        text-color="white"
                        size="sm"
                        label="Non évalué"/>
                      <q-btn v-else
                        @click="setStatus(evaluation.id, 'Empty')"
                        color="white"
                        text-color="black"
                        size="sm"
                        label="Non évalué"/>
                    </td>
                    <td>
                      <q-btn v-if="evaluation.status === 'InProgress'"
                        :color="statusColor('InProgress')"
                        text-color="white"
                        size="sm"
                        label="En cours"/>
                      <q-btn v-else
                        @click="setStatus(evaluation.id, 'InProgress')"
                        color="white"
                        text-color="black"
                        size="sm"
                        label="En cours"/>
                    </td>
                    <td>
                      <q-btn v-if="evaluation.status === 'Acquired'"
                        :color="statusColor('Acquired')"
                        text-color="white"
                        size="sm"
                        label="Acquis"/>
                      <q-btn v-else
                        @click="setStatus(evaluation.id, 'Acquired')"
                        color="white"
                        text-color="black"
                        size="sm"
                        label="Acquis"/>
                    </td>
                    <td>
                      <q-btn v-if="evaluation.status === 'NotAcquired'"
                        :color="statusColor('NotAcquired')"
                        text-color="white"
                        size="sm"
                        label="Non acquis"/>
                      <q-btn v-else
                        @click="setStatus(evaluation.id, 'NotAcquired')"
                        color="white"
                        text-color="black"
                        size="sm"
                        label="Non acquis"/>
                    </td>
                    <td>
                      <q-input filled autogrow
                        debounce="500"
                        :value="evaluation.comment"
                        @input="setComment(evaluation.id, $event)"/>
                    </td>
                  </tr>
                </tbody>
              </q-markup-table>
            </q-card-section>
          </q-card>
        </div>
      </div>
      <div class="col">
      </div>
    </div>
    <div class="row q-mb-md q-mt-xl">
      <div class="col">
      </div>
      <div class="col-auto">
        <q-btn color="primary" text-color="white"
          label="Sauvegarder" icon="save" size="xl"
          @click="save"/>
      </div>
      <div class="col">
      </div>
    </div>
  </div>
</template>

<script>
import { STATUS_COLORS } from '../pages/status-color.js'
export default {
  props: {
    detail: Object
  },
  data () {
    return {
    }
  },
  methods: {
    statusColor: function (status) {
      return STATUS_COLORS[status]
    },
    setStatus: function (id, status) {
      this.$store.dispatch('evaluation/setStatus', { id: id, status: status })
    },
    setComment: function (id, value) {
      this.$store.dispatch('evaluation/setComment', { id: id, comment: value })
    },
    save: async function () {
      this.$emit('save')
    }
  }
}
</script>
<style>
.pre-wrap {
  white-space: pre-wrap;
}
</style>
