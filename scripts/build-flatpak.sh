#!/bin/sh

flatpak-builder --repo=repo app/ data/studio.planetpeanut.Twinkle.yml
flatpak run studio.planetpeanut.Twinkle
