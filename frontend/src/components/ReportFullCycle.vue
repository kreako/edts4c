<template>
  <div>
    <div v-for="eleve in report" :key="eleve.id">
      <div>
        <span class="text-subtitle2">Prénom :</span> {{ eleve.firstname }}
      </div>
      <div>
        <span class="text-subtitle2">Nom :</span> {{ eleve.lastname }}
      </div>
      <div>
        <span class="text-subtitle2">Date de naissance :</span> {{ eleve.birthdate }}
      </div>
      <div>
        <span class="text-subtitle2">Date d'entrée à l'école :</span> {{ eleve.school_entry }}
      </div>
      <div>
        <span class="text-subtitle2">Date de l'évaluation :</span> {{ eleve.evaluation_date }}
      </div>
      <div>
        <span class="text-subtitle2">Cycle :</span> {{ cycle }}
      </div>
      <div v-for="domain in eleve.domains" :key="domain.id">
        <div class="text-subtitle2 q-mt-md">
          {{ domain.rank }}. {{ domain.title }}
        </div>
        <div v-for="component in domain.components" :key="component.id">
          <div class="q-mt-md q-ml-md">
            {{ component.rank }}. {{ component.title }}
          </div>
          <div v-for="competency in component.competencies" :key="competency.id">
            <div class="q-mt-md q-ml-xl row">
              <div>
                {{ competency.rank }}. {{ competency.title }}
              </div>
              <div>
                <q-badge class="q-ml-md" v-if="statusDisplay(competency.status)" :color="statusColor(competency.status)">
                  {{ statusLabel(competency.status) }}
                </q-badge>
              </div>
              <div class="text-caption q-ml-md pre-wrap">{{ competency.comment }}</div>
            </div>
          </div>
        </div>
      </div>
      <div class="text-subtitle2 q-mt-md">
        Commentaires
      </div>
      <div class="pre-wrap q-ml-md">{{ eleve.general_comment }}</div>
      <hr class="q-my-md"/>
    </div>
  </div>
</template>

<script>
import { STATUS_COLORS } from '../pages/status-color.js'
export default {
  props: {
    report: Array,
    cycle: String
  },
  methods: {
    statusColor: function (status) {
      return STATUS_COLORS[status]
    },
    statusLabel: function (status) {
      if (status === 'Empty') {
        return ''
      } else if (status === 'InProgress') {
        return 'En cours'
      } else if (status === 'Acquired') {
        return 'Acquis'
      } else if (status === 'NotAcquired') {
        return 'Non acquis'
      }
    },
    statusDisplay: function (status) {
      if (status === 'Empty') {
        return false
      }
      return true
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
