<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div v-if="$route.params.type === 'socle'">
        <ReportFullSocle :loading="loading" :socle="socle"/>
      </div>
      <div v-if="isCycle">
        <div v-if="$route.params.detail === 'summary'">
          <ReportSummaryCycle :report="report" :cycle="$route.params.type"/>
        </div>
        <div v-else>
          <ReportFullCycle :report="report" :cycle="$route.params.type"/>
        </div>
      </div>
    </div>
  </q-page>
</template>

<script>
import { mapState } from 'vuex'
import ReportFullSocle from '../components/ReportFullSocle.vue'
import ReportSummaryCycle from '../components/ReportSummaryCycle.vue'
import ReportFullCycle from '../components/ReportFullCycle.vue'

export default {
  components: {
    ReportFullSocle, ReportSummaryCycle, ReportFullCycle
  },
  computed: {
    ...mapState({
      loading: state => state.report.loading,
      socle: state => state.report.socle,
      report: state => state.report.report
    }),
    isCycle: function () {
      return ['C1', 'C2', 'C3', 'C4'].includes(this.$route.params.type)
    }
  },
  mounted: function () {
    this.$store.dispatch('report/loadSocle')
    if (this.isCycle) {
      if (this.$route.params.detail === 'summary') {
        this.$store.dispatch('report/loadReportSummary', this.$route.params.type)
      } else {
        this.$store.dispatch('report/loadReportFull', this.$route.params.type)
      }
    }
  }
}
</script>
<style>
.pre-wrap {
  white-space: pre-wrap;
}
</style>
