import 'zx/globals'

await $`tsc`

await $`electron-builder --x64 --arm64`
