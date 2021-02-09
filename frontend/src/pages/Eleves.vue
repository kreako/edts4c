<template>
  <q-page padding>
    <div v-if="loading" class="flex flex-center q-mt-xl">
      <q-spinner-gears color="primary" size="xl"/>
    </div>
    <div v-else>
      <div class="row">
        <div class="col">
        </div>
        <div class="col-auto">
          <q-markup-table data-test-id="eleves-table">
            <thead>
              <tr>
                <td>
                  Prénom
                </td>
                <td>
                  Nom
                </td>
                <td>
                  Date de naissance
                </td>
                <td>
                  Entrée à l'école
                </td>
                <td>
                  Cycle
                </td>
                <td>
                </td>
              </tr>
            </thead>
            <tbody>
              <tr v-for="eleve in filteredEleves" :key="eleve.id">
                <td>
                  <div v-if="inEdit === eleve.id">
                    <q-input v-model="firstnameEdit" @keydown.enter="save(eleve.id)" filled/>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    {{ eleve.firstname }}
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
                <td>
                  <div v-if="inEdit === eleve.id">
                    <q-input v-model="lastnameEdit" @keydown.enter="save(eleve.id)" filled/>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    {{ eleve.lastname }}
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
                <td>
                  <div v-if="inEdit === eleve.id">
                    <q-input v-model="birthdateEdit" mask="####-##-##" fill-mask @keydown.enter="save(eleve.id)" filled>
                      <template v-slot:append>
                        <q-icon name="event" class="cursor-pointer">
                          <q-popup-proxy transition-show="scale" transition-hide="scale">
                            <q-date v-model="birthdateEdit" mask="YYYY-MM-DD">
                              <div class="row items-center justify-end">
                                <q-btn v-close-popup label="OK" color="primary" flat />
                              </div>
                            </q-date>
                          </q-popup-proxy>
                        </q-icon>
                      </template>
                    </q-input>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    {{ eleve.birthdate }}
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
                <td>
                  <div v-if="inEdit === eleve.id">
                    <q-input v-model="schoolEntryEdit" mask="####-##-##" fill-mask @keydown.enter="save(eleve.id)" filled>
                      <template v-slot:append>
                        <q-icon name="event" class="cursor-pointer">
                          <q-popup-proxy transition-show="scale" transition-hide="scale">
                            <q-date v-model="schoolEntryEdit" mask="YYYY-MM-DD">
                              <div class="row items-center justify-end">
                                <q-btn v-close-popup label="OK" color="primary" flat />
                              </div>
                            </q-date>
                          </q-popup-proxy>
                        </q-icon>
                      </template>
                    </q-input>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    {{ eleve.school_entry }}
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
                <td>
                  <div v-if="inEdit === eleve.id">
                    <q-btn-toggle v-model="cycleEdit" text-color="black" :options="cycleEditOptions"/>
                    <span v-if="eleve.cycle !== eleve.estimated_cycle">
                      <span class="q-mx-xs">(</span>
                      <q-badge :color="cycleColor(eleve.estimated_cycle)">
                        {{ eleve.estimated_cycle }}
                      </q-badge>
                      <span class="q-mx-xs">)</span>
                    </span>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    <q-badge :color="cycleColor(eleve.cycle)">
                      {{ eleve.cycle }}
                    </q-badge>
                    <span v-if="eleve.cycle !== eleve.estimated_cycle">
                      <span class="q-mx-xs">(</span>
                      <q-badge :color="cycleColor(eleve.estimated_cycle)">
                        {{ eleve.estimated_cycle }}
                      </q-badge>
                      <span class="q-mx-xs">)</span>
                    </span>
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
                <td class="text-right">
                  <div v-if="inEdit === eleve.id">
                    <q-btn color="primary" text-color="white" icon="save" label="Sauvegarder" size="sm" @click="save(eleve.id)"/>
                  </div>
                  <div v-if="inEdit !== eleve.id && saved(eleve.id)">
                    <q-btn flat dense round color="grey-5" icon="more_vert" data-test-id="eleves-more">
                      <q-menu>
                        <q-list>
                          <q-item clickable v-close-popup @click="edit(eleve.id)">
                            <q-item-section avatar>
                              <q-icon name="edit" size="sm"/>
                            </q-item-section>
                            <q-item-section>
                              Modifier
                            </q-item-section>
                          </q-item>
                          <q-item clickable v-close-popup @click="remove(eleve.id)">
                            <q-item-section avatar>
                              <q-icon name="delete" size="sm"/>
                            </q-item-section>
                            <q-item-section>
                              Supprimer
                            </q-item-section>
                          </q-item>
                        </q-list>
                      </q-menu>
                    </q-btn>
                  </div>
                  <div v-if="inEdit !== eleve.id && !saved(eleve.id)">
                    <q-spinner-gears color="primary"/>
                  </div>
                </td>
              </tr>
            </tbody>
          </q-markup-table>
        </div>
        <div class="col-auto q-ml-lg">
          <q-card flat bordered class="bg-grey-1">
            <q-card-section>
              Filtres
            </q-card-section>
            <q-separator inset />
            <q-card-section>
              <div class="column">
                <q-toggle v-model="showC1" :color="cycleColor('C1')" label="C1" left-label size="xs"/>
                <q-toggle v-model="showC2" :color="cycleColor('C2')" label="C2" left-label size="xs"/>
                <q-toggle v-model="showC3" :color="cycleColor('C3')" label="C3" left-label size="xs"/>
                <q-toggle v-model="showC4" :color="cycleColor('C4')" label="C4" left-label size="xs"/>
              </div>
            </q-card-section>
          </q-card>
        </div>
        <div class="col">
        </div>
      </div>
      <div class="flex flex-center q-my-xl">
        <q-btn color="primary" text-color="white" icon="add_box" label="Nouvel élève" @click="showNewDialog" data-test-id="eleves-new"/>
      </div>
    </div>
    <q-dialog v-model="newDialogShow">
      <q-card>
        <q-card-section>
          <div class="text-h6">
            Nouvel élève
          </div>
        </q-card-section>
        <q-item-label header>
            Prénom
        </q-item-label>
        <q-item dense>
          <q-input v-model="firstnameNew" data-test-id="eleves-new-firstname"/>
        </q-item>
        <q-item-label header>
            Nom
        </q-item-label>
        <q-item dense>
          <q-input v-model="lastnameNew" data-test-id="eleves-new-lastname"/>
        </q-item>
        <q-item-label header>
            Date de naissance
        </q-item-label>
        <q-item dense>
          <q-input v-model="birthdateNew" mask="####-##-##" fill-mask hint="YYYY-MM-DD" data-test-id="eleves-new-birthdate">
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy transition-show="scale" transition-hide="scale">
                  <q-date v-model="birthdateNew" mask="YYYY-MM-DD">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="OK" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </q-item>
        <q-item-label header>
            Date d'entrée à l'école
        </q-item-label>
        <q-item dense>
          <q-input v-model="schoolEntryNew" mask="####-##-##" fill-mask hint="YYYY-MM-DD" data-test-id="eleves-new-schoolentry">
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy transition-show="scale" transition-hide="scale">
                  <q-date v-model="schoolEntryNew" mask="YYYY-MM-DD">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="OK" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </q-item>
        <q-item-label header>
            Cycle
        </q-item-label>
        <q-item dense>
          <q-btn-toggle v-model="cycleNew" text-color="black" :options="cycleEditOptions" data-test-id="eleves-new-cycle"/>
        </q-item>
        <q-separator/>
        <q-card-actions align="right">
          <q-btn flat @click="newEleve" data-test-id="eleves-new-save">
            Sauvegarder
          </q-btn>
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script>
import { mapState } from 'vuex'
import { CYCLE_COLORS, CYCLE_COLORS_BG } from './cycle-color.js'

export default {
  data () {
    return {
      newDialogShow: false,
      firstnameEdit: '',
      lastnameEdit: '',
      birthdateEdit: '',
      schoolEntryEdit: '',
      cycleEdit: '',
      cycleEditOptions: [
        { label: 'C1', value: 'C1', color: CYCLE_COLORS_BG.C1 },
        { label: 'C2', value: 'C2', color: CYCLE_COLORS_BG.C2 },
        { label: 'C3', value: 'C3', color: CYCLE_COLORS_BG.C3 },
        { label: 'C4', value: 'C4', color: CYCLE_COLORS_BG.C4 }
      ],
      firstnameNew: '',
      lastnameNew: '',
      birthdateNew: '',
      schoolEntryNew: '',
      cycleNew: '',
      showC1: true,
      showC2: true,
      showC3: true,
      showC4: true,
      inEdit: -1,
      inSave: -1
    }
  },
  computed: {
    ...mapState({
      loading: state => state.eleves.loading,
      eleves: state => state.eleves.eleves
    }),
    filteredEleves: function () {
      return this.eleves
        .filter(eleve => (this.showC1 && eleve.cycle === 'C1') || (eleve.cycle !== 'C1'))
        .filter(eleve => (this.showC2 && eleve.cycle === 'C2') || (eleve.cycle !== 'C2'))
        .filter(eleve => (this.showC3 && eleve.cycle === 'C3') || (eleve.cycle !== 'C3'))
        .filter(eleve => (this.showC4 && eleve.cycle === 'C4') || (eleve.cycle !== 'C4'))
    }
  },
  methods: {
    edit: function (id) {
      this.inEdit = id
      for (const e of this.eleves) {
        if (e.id === id) {
          this.firstnameEdit = e.firstname
          this.lastnameEdit = e.lastname
          this.birthdateEdit = e.birthdate
          this.schoolEntryEdit = e.school_entry
          this.cycleEdit = e.cycle
        }
      }
    },
    save: async function (id) {
      this.inSave = id
      this.inEdit = -1
      await this.$store.dispatch('eleves/update', { id: id, firstname: this.firstnameEdit, lastname: this.lastnameEdit, birthdate: this.birthdateEdit, schoolEntry: this.schoolEntryEdit, cycle: this.cycleEdit })
      await this.$store.dispatch('eleves/loadById', id)
    },
    showNewDialog: function () {
      this.newDialogShow = true
      this.firstnameNew = ''
      this.lastnameNew = ''
      this.birthdateNew = ''
      this.schoolEntryNew = ''
      this.cycleNew = ''
    },
    newEleve: async function () {
      this.newDialogShow = false
      await this.$store.dispatch('eleves/newEleve',
        {
          firstname: this.firstnameNew,
          lastname: this.lastnameNew,
          birthdate: this.birthdateNew,
          schoolEntry: this.schoolEntryNew,
          cycle: this.cycleNew
        })
    },
    remove: async function (id) {
      await this.$store.dispatch('eleves/remove', id)
      await this.$store.dispatch('eleves/load')
    },
    saved: function (id) {
      if (this.inSave === id) {
        for (const e of this.eleves) {
          if (e.id === id) {
            if (e.firstname === this.firstnameEdit &&
                e.lastname === this.lastnameEdit &&
                e.birthdate === this.birthdateEdit &&
                e.school_entry === this.schoolEntryEdit &&
                e.cycle === this.cycleEdit) {
              this.inSave = -1
              return true
            } else {
              return false
            }
          }
        }
      }
      return true
    },
    cycleColor: function (cycle) {
      return CYCLE_COLORS[cycle]
    }
  },
  mounted: function () {
    this.$store.dispatch('eleves/load')
  }
}
</script>
