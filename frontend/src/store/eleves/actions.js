import axios from 'axios'

export async function load (context) {
  context.commit('setLoading', true)
  try {
    const response = await axios.get('/api/eleves/all')
    const eleves = response.data.eleves
    context.commit('setEleves', eleves)
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}

export async function loadById (context, id) {
  try {
    const response = await axios.get(`/api/eleves/by_id/${id}`)
    const eleve = response.data.eleve
    context.commit('setEleve', eleve)
  } catch (error) {
    // TODO
    console.error(error)
  }
}

export async function update (context, { id, firstname, lastname, birthdate, schoolEntry, cycle }) {
  // TODO error ?
  await axios.put(`/api/eleves/update/${id}`, {
    firstname: firstname,
    lastname: lastname,
    birthdate: birthdate,
    school_entry: schoolEntry,
    cycle: cycle
  })
}

export async function remove (context, id) {
  // TODO error ?
  await axios.put(`/api/eleves/set_active/${id}`, { active: false })
}

export async function newEleve (context, { firstname, lastname, birthdate, schoolEntry, cycle }) {
  context.commit('setLoading', true)
  try {
    // TODO post error
    await axios.post('/api/eleves/new', {
      firstname: firstname,
      lastname: lastname,
      birthdate: birthdate,
      school_entry: schoolEntry,
      cycle: cycle
    })
    await context.dispatch('load')
  } catch (error) {
    // TODO
    console.error(error)
  }
  context.commit('setLoading', false)
}
