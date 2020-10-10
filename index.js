const rust=import('./pkg')

rust
  .then(m=> m.greet('sunny'))
  .catch(console.error)
