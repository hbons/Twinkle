project(
  'Twinkle',
  'rust',
  version: '50.0.0',
  meson_version: '>=1.1'
)

command_name = 'twinkle'
configuration = configuration_data()
configuration.set('TWINKLE_APP_ID', 'studio.planetpeanut.Twinkle')


gnome = import('gnome')
dependency('gtk4')
dependency('libadwaita-1')
# dependency('libsecret-1')  # TODO

subdir('data')
subdir('data/icons')
subdir('src')

summary(
  {
    'bindir': get_option('bindir'),
    'devel': get_option('devel'),
    'features': get_option('features'),
  },
  section: 'Options',
)


cargo = find_program('cargo', required: true)

custom_target(
  command_name,
  command: [
    'sh', '-c',
    'cd ' + meson.project_source_root() + ' && ' +
    'cargo build --release --features=' + get_option('features') + ' && ' +
    'cp target/release/' + command_name + ' ' + meson.project_build_root() + '/' + command_name
  ],
  output: command_name,
  build_by_default: true,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
)

test('cargo_test',
  cargo,
  args: ['test'],
  workdir: meson.project_source_root(),
  timeout: 300,
)


gnome.post_install(
    # glib_compile_schemas: true,
    # gtk_update_icon_cache: true,
    update_desktop_database: true,
)
