import { defineConfig, presetIcons, presetTypography, presetUno } from 'unocss'

export default defineConfig({
  presets: [presetUno(), presetTypography(), presetIcons()],
  theme: {
    colors: {
      ink: '#14213d',
      amber: '#fca311',
      mist: '#e5ecf4',
      frost: '#f8fbff',
      slate: '#52606d'
    }
  }
})

