# [Twinkle](https://github.com/hbons/twinkle)

*Automatically sync your files with Git*

![The Twinkle app icon](data/icons/scalable/studio.planetpeanut.Twinkle.svg)

## Sync with Git providers or host your own

Twinkle is powered by the version control system *Git* and supports the *Large File Storage (LFS)* extension. Collaborate using services like *GitHub*, *GitLab* and *Codeberg* or self-host.


## Install on Linux

Twinkle is designed and built for the *GNOME* platform. Available on [Flathub](https://flathub.org/).

```shell
# Install from Flathub
flatpak remote-add flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install flathub studio.planetpeanut.Twinkle
```


## Build from source

Twinkle is *Free and Open Source* software: you can redistribute it and/or modify it under the terms of the *GNU General Public License v3* or later. It's written in *Rust* and here's how to build it:

```shell
# Build with Meson
meson setup build
ninja -C build
sudo ninja install -C build
```


## Links

* [@hbons@mastodon.social](https://mastodon.social/@hbons)
* [planetpeanut.studio](https://planetpeanut.studio)

<br>
Have fun, make awesome! :)
<br>
<br>
– Hylke
