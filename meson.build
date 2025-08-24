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
  command_name,
  command: [
    'sh', '-c',
    'cd ' + meson.project_source_root() + ' && ' +
    'cargo build --release && ' +
    'cp target/release/' + command_name + ' ' + meson.project_build_root() + '/' + command_name
  ],
  output: command_name,
  build_by_default: true,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
)


# gnome.post_install(
#   gtk_update_icon_cache: true,
#   update_desktop_database: true,
# )


summary(
  {
    'nightly': get_option('nightly'),
    'bindir': get_option('bindir')
  },
  section: 'Options',
)
