import axios from 'axios'

export async function load (context) {
  context.commit('setLoading', true)
  const response = await axios.get('/api/eleves/all')
  const eleves = response.data.eleves
  context.commit('setEleves', eleves)
  context.commit('setLoading', false)
}

export async function loadById (context, id) {
  const response = await axios.get(`/api/eleves/by_id/${id}`)
  const eleve = response.data.eleve
  context.commit('setEleve', eleve)
}

export async function update (context, { id, firstname, lastname, birthdate, schoolEntry, cycle }) {
  await axios.put(`/api/eleves/update/${id}`, {
    firstname: firstname,
    lastname: lastname,
    birthdate: birthdate,
    school_entry: schoolEntry,
    cycle: cycle
  })
}

export async function remove (context, id) {
  await axios.put(`/api/eleves/set_active/${id}`, { active: false })
}

export async function newEleve (context, { firstname, lastname, birthdate, schoolEntry, cycle }) {
  context.commit('setLoading', true)
  await axios.post('/api/eleves/new', {
    firstname: firstname,
    lastname: lastname,
    birthdate: birthdate,
    school_entry: schoolEntry,
    cycle: cycle
  })
  await context.dispatch('load')
  context.commit('setLoading', false)
}
