import fs from 'node:fs'

const wasmBuff=fs.readFileSync('./sample.wasm')

const {instance, module}=await WebAssembly.instantiate(wasmBuff)

const res=instance.exports.how_old(2025, 1991)
console.log('how old: ', res)
