import init, { greet } from './lib/pkg/wasm_workshop.js'

init().then(() => {
  document.querySelector('button').addEventListener('click', e => {
    greet()
  })
})
