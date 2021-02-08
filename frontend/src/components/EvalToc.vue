<template>
  <q-markup-table>
    <thead>
      <tr>
        <td>
        </td>
        <td>
        </td>
        <td>
          Non évalués
        </td>
        <td>
          En cours
        </td>
        <td>
          Acquis
        </td>
        <td>
          Non acquis
        </td>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(t, i) in toc" :key="i">
        <td v-if="t.kind === 'Domain'" colspan="6">
          {{ t.data.rank }}. {{ t.data.title }}
        </td>
        <td v-if="t.kind === 'Component'">
          &nbsp;
        </td>
        <td v-if="t.kind === 'Component'">
          <router-link :to="`/evaluation/${$route.params.cycle}/${t.data.id}`" class="clean-link">
            {{ t.data.rank }}. {{ t.data.title }}
          </router-link>
        </td>
        <td v-if="t.kind === 'Component'">
          <LabeledProgress color="deep-orange-14" :percent="t.data.nb_empty_percent"/>
        </td>
        <td v-if="t.kind === 'Component'">
          <LabeledProgress color="blue-14" :percent="t.data.nb_in_progress_percent"/>
        </td>
        <td v-if="t.kind === 'Component'">
          <LabeledProgress color="green-14" :percent="t.data.nb_acquired_percent"/>
        </td>
        <td v-if="t.kind === 'Component'">
          <LabeledProgress color="pink-14" :percent="t.data.nb_not_acquired_percent"/>
        </td>
      </tr>
      <tr>
        <td>
          <router-link :to="`/evaluation/${$route.params.cycle}/end`" class="clean-link">
            Commentaire général
          </router-link>
        </td>
      </tr>
    </tbody>
  </q-markup-table>
</template>

<script>
import LabeledProgress from './LabeledProgress.vue'
export default {
  props: {
    toc: Array
  },
  components: {
    LabeledProgress
  },
  data () {
    return {}
  }
}
</script>
