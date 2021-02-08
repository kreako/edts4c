<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row q-ml-md q-mt-md">
        <q-breadcrumbs>
          <q-breadcrumbs-el icon="home" to="/socle/" />
          <q-breadcrumbs-el
            :label="`${domain.rank}. ${domain.title.slice(0, 30)}`"
            :to="`/socle/domain/${$route.params.domainId}`" />
          <q-breadcrumbs-el
            :label="`${component.rank}. ${component.title.slice(0, 30)}`"
            :to="`/socle/domain/${$route.params.domainId}/component/${$route.params.componentId}`" />
          <q-breadcrumbs-el
            :label="`${competency.rank}. ${competency.title}`"
            :to="`/socle/domain/${$route.params.domainId}/component/${$route.params.componentId}/competency/${$route.params.competencyId}`" />
        </q-breadcrumbs>
      </div>
      <div class="row q-ml-md q-mt-xl">
        <div class="col-auto">
          <q-list bordered>
            <q-item class="show-children-on-hover">
              <q-item-section avatar top>
                <q-avatar rounded color="grey-4" text-color="dark" size="sm">
                  {{ competency.rank }}
                </q-avatar>
              </q-item-section>
              <q-item-section v-if="inEdit">
                <q-input v-model="editTitle" @keydown.enter="save" filled dense/>
                <q-btn color="primary" text-color="white"
                  icon="save" label="Sauvegarder"
                  size="sm" @click="save"/>
              </q-item-section>
              <q-item-section v-else>
                {{ competency.title }}
              </q-item-section>
              <q-item-section side top>
                <q-btn flat round class="q-ml-md show-on-hover"
                  text-color="grey-9" icon="edit" size="xs"
                  @click="edit"/>
              </q-item-section>
            </q-item>
            <q-separator spaced />
            <CompetencyCycleDetail
              cycle="C1" :competency="competency"
              @save="saveDetail"/>
            <q-separator spaced />
            <CompetencyCycleDetail
              cycle="C2" :competency="competency"
              @save="saveDetail"/>
            <q-separator spaced />
            <CompetencyCycleDetail
              cycle="C3" :competency="competency"
              @save="saveDetail"/>
            <q-separator spaced />
            <CompetencyCycleDetail
              cycle="C4" :competency="competency"
              @save="saveDetail"/>
          </q-list>
        </div>
        <div class="col">
        </div>
      </div>
    </div>
  </q-page>
</template>

<script>
import CompetencyCycleDetail from '../components/CompetencyCycleDetail.vue'
import { mapState } from 'vuex'
export default {
  components: {
    CompetencyCycleDetail
  },
  data () {
    return {
      inEdit: false,
      editTitle: ''
    }
  },
  computed: {
    ...mapState({
      loading: state => state.socle.loading,
      domain: state => state.socle.domain,
      component: state => state.socle.component,
      competency: state => state.socle.competency
    })
  },
  methods: {
    saveDetail: function (data) {
      this.$store.dispatch('socle/setCompetencyCycle', { competencyId: this.competency.id, cycle: data.cycle, text: data.detail })
    },
    edit: function () {
      this.inEdit = true
      this.editTitle = this.competency.title
    },
    save: function () {
      this.inEdit = false
      this.$store.dispatch('socle/setCompetencyTitle', { competencyId: this.competency.id, title: this.editTitle })
    }
  },
  mounted: function () {
    this.$store.dispatch('socle/loadCompetency', this.$route.params.competencyId)
  }
}
</script>
<style lang="scss">
.show-children-on-hover .show-on-hover {
  visibility: hidden;
}
.show-children-on-hover:hover .show-on-hover {
  visibility: visible;
}
.show-children-on-hover .hide-on-hover {
  visibility: visible;
}
.show-children-on-hover:hover .hide-on-hover {
  visibility: hidden;
}
</style>
