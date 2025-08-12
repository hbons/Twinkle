project('Twinkle', 'rust', version: '48.0.0')

command_name = 'twinkle'
configuration = configuration_data()
configuration.set('TWINKLE_APP_ID', 'studio.planetpeanut.Twinkle')

gnome = import('gnome')
dependency('gtk4')
dependency('libadwaita-1')
dependency('libsecret-1')

subdir('data')
subdir('data/icons')
subdir('src')


cargo = find_program('cargo', required: true)
cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)


cargo_build = custom_target(
  binary_name,
  command: [
    'sh', '-c',
    'cd ' + meson.source_root() + ' && ' +
    'cargo build --release && ' +
    'cp target/release/' + binary_name + ' ' + meson.build_root() + '/' + binary_name
  ],
  output: binary_name,
  build_by_default: true,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
)


gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)


summary(
  { 'nightly': get_option('nightly'), },
  section: 'Options',
)
