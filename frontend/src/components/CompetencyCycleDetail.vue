<template>
  <q-item class="show-children-on-hover">
    <q-item-section>
      <q-item-label overline class="text-weight-bolder">
        {{ cycle }}
      </q-item-label>
      <q-item-label>
        <div v-if="inEdit">
          <div class="column">
            <q-input v-model="editDetail" type="textarea" autogrow/>
            <q-btn color="primary" text-color="white"
              icon="save" label="Sauvegarder" size="sm"
              @click="save"/>
          </div>
        </div>
        <div v-else>
          <q-item-label class="pre-wrap">{{ detail }}</q-item-label>
        </div>
      </q-item-label>
    </q-item-section>
    <q-item-section side top>
      <q-item-label caption v-if="inEdit">
        <q-btn flat round class="q-ml-md"
          text-color="grey-9" icon="save" size="xs"
          @click="save"/>
      </q-item-label>
      <q-item-label caption v-else>
        <q-btn flat round class="q-ml-md show-on-hover"
          text-color="grey-9" icon="edit" size="xs"
          @click="edit"/>
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script>
export default {
  props: {
    cycle: String,
    competency: Object
  },
  data () {
    return {
      inEdit: false,
      editDetail: ''
    }
  },
  computed: {
    detail: function () {
      if (this.cycle === 'C1') {
        return this.competency.c1
      } else if (this.cycle === 'C2') {
        return this.competency.c2
      } else if (this.cycle === 'C3') {
        return this.competency.c3
      } else if (this.cycle === 'C4') {
        return this.competency.c4
      } else {
        throw new Error(`Invalid cycle : ${this.cycle}`)
      }
    }
  },
  methods: {
    edit: function () {
      this.inEdit = true
      this.editDetail = this.detail
    },
    save: function () {
      this.inEdit = false
      this.$emit('save', { cycle: this.cycle, detail: this.editDetail })
    }
  }
}
</script>
<style lang="scss">
.pre-wrap {
  white-space: pre-wrap;
}
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
