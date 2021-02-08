<template>
  <div>
    <div class="row q-mb-md q-mt-xl">
      <div class="col">
      </div>
      <div class="col-auto">
        <div class="text-h6">
          Commentaires généraux
        </div>
      </div>
      <div class="col">
      </div>
    </div>
    <div class="row q-mb-md q-mt-xl">
      <div class="col">
      </div>
      <div class="col-auto">
        <div v-for="c in comment" :key="c.id">
          <q-card class="q-mt-md">
            <q-card-section>
              <div class="text-subtitle2">
                {{ c.eleve_firstname }} {{ c.eleve_lastname }}
              </div>
            </q-card-section>
            <q-separator/>
            <q-card-section>
              <q-input filled type="textarea"
                debounce="500"
                :value="c.comment"
                @input="setComment(c.id, $event)"/>
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
export default {
  props: {
    cycle: String,
    comment: Array
  },
  data () {
    return {}
  },
  methods: {
    setComment: function (id, value) {
      this.$store.dispatch('evaluation/setGeneralComment', { id: id, comment: value })
    },
    save: async function () {
      this.$emit('save')
    }
  }
}
</script>
