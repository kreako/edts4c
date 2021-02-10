// import something here

// "async" is optional;
// more info on params: https://quasar.dev/quasar-cli/boot-files
export default async ({ store }) => {
  // something to do
  store.subscribeAction({
    error: (action, state, error) => {
      console.log(`error action ${action.type}`)
      console.log('error', action)
      console.error(error)
      store.dispatch('error/setError', `Action: ${JSON.stringify(action, null, 2)}\nError: ${JSON.stringify(error, null, 2)}`)
    }
  })
}
