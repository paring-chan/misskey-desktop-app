import 'zx/globals'

await $`tsc`

$.env.DEBUG = 'true'

await $`electron dist/index.js`
