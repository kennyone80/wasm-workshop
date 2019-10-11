import init, { greet } from './lib/pkg/wasm_workshop.js'

init().then(() => {
  document.querySelector('button').addEventListener('click', e => {
    let name = document.querySelector('input').value
    greet(name)
  })
})
