import Vue from 'vue'
import Vuex from 'vuex'

import socle from './socle'
import eleves from './eleves'
import evaluations from './evaluations'
import evaluation from './evaluation'
import report from './report'

Vue.use(Vuex)

/*
 * If not building with SSR mode, you can
 * directly export the Store instantiation;
 *
 * The function below can be async too; either use
 * async/await or return a Promise which resolves
 * with the Store instance.
 */

export default function (/* { ssrContext } */) {
  const Store = new Vuex.Store({
    modules: {
      socle, eleves, evaluations, evaluation, report
    },

    // enable strict mode (adds overhead!)
    // for dev mode only
    strict: process.env.DEBUGGING
  })

  return Store
}