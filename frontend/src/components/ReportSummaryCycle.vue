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
          <div class="row q-ml-xl q-gutter-md">
              <div>
                En cours :
                <q-badge :color="statusColor('InProgress')">
                  {{ component.nb_in_progress }}
                </q-badge>
              </div>
              <div>
                Acquis :
                <q-badge :color="statusColor('Acquired')">
                  {{ component.nb_acquired }}
                </q-badge>
              </div>
              <div>
                Non acquis :
                <q-badge :color="statusColor('NotAcquired')">
                  {{ component.nb_not_acquired }}
                </q-badge>
              </div>
          </div>
          <div class="q-ml-xl">
            <div v-for="comment in component.comments" :key="comment" class="text-caption pre-wrap">{{ comment }}</div>
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
