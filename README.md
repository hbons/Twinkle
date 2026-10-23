![The SparkleShare app icon](data/icons/scalable/studio.planetpeanut.Twinkle.svg)

# [Twinkle](https://sparkleshare.org) ～ *Sync files with Git*

The command-line version of [SparkleShare](https://sparkleshare.org). Sync your files with any Git repository. Collaborate using services like [GitHub](https://github.com), [GitLab](https://gitlab.com) and [Codeberg](https://codeberg.org) or host your own using [Forgejo](https://forgejo.org). No special server software required and its only dependencies are `openssh`, `git`, and (optionally) `git-lfs`.

<br>

[![Flatpak](https://github.com/hbons/Twinkle/actions/workflows/tests.yml/badge.svg)](https://github.com/hbons/Twinkle/actions/workflows/tests.yml)

<br>


## 1. Install on Linux / macOS

```shell
cargo install --path .
```

Or get the [nightly Flatpak](https://github.com/hbons/Twinkle/releases/tag/flatpak-nightly).

```shell
Usage: twinkle <command> [args…]

Commands:
    clone <user@host:path> [path]
    init  <user@host:path> [path]
    sync  [path] [--interval=60]

Support:
    sparkleshare.org/support

Options:
    --help, --version, --deps, --env
```

<br>


## 2. Setup

> [!CAUTION]
> ***Twinkle is still in preview***. Data loss is unlikely but possible. Run commands with `DEBUG=1 twinkle <COMMAND>` to see what's going on under the hood and run `twinkle check` to diagnose setup problems.


### 2.1. Starting with a remote repository

```sh
twinkle clone ssh://git@codeberg.org/user/repo

cd repo
git config --local user.name "Your Name"
git config --local user.email "your@email"

twinkle sync
```

### 2.2. Starting with local files

```sh
twinkle init ssh://git@codeberg.org/user/repo

git config --local user.name "Your Name"
git config --local user.email "your@email"

twinkle sync
```

<br>


## 3. Git LFS

Twinkle supports the [Large File Storage](https://git-lfs.com) (LFS) extension. When a file larger than `size_threshold` is added, `git lfs track <FILE>` is called automatically. To enable LFS:

```sh
git lfs --version  # Check if git-lfs is installed

git config sync.lfs.enabled true
git config sync.lfs.sizeThreshold 3m  # (k)b / (m)b / (g)b

twinkle sync
```

In addition to the threshold, you can still add your own patterns to `.gitattributes`:

```sh
git lfs track *.mp4
```

<br>


## 4. Merge Strategy

> [!IMPORTANT]
> Twinkle keeps a ***linear commit history*** by using `git-rebase` to merge commits.

All files are [treated as binary](https://git-scm.com/docs/gitattributes#_marking_files_as_binary), so during merge conflicts Git won't try to resolve anything. Instead, both diverging versions of the file are checked out next to the original, so the users of the repository can decide what to keep.

Here's how it works when *Alice* and then *Bob* both make changes to `README.md` at the same time. Let's assume Alice's changes reach the repository first:

* Alice adds, Bob adds:
  * `README (Alice).md` and Bob's `README.md`
* Alice adds/modifies, Bob modifies:
  * Original `README.md`, `README (Alice).md` and `README (Bob).md`
* Alice modifies, Bob adds/modifies:
  * Original `README.md`, `README (Alice).md` and `README (Bob).md`
* Alice adds/modifies, Bob deletes:
  * Alice's `README.md`
* Alice deletes, Bob adds/modifies:
  * Bob's `README.md`

When syncing between clients with the same user name:

* Original `README.md`, `README (A).md` and `README (B).md`

<br>


## 5. Configuration

> [!IMPORTANT]
> For predictability and portability reasons, Twinkle
> ***ignores the global and system Git configuration files***.
> Settings are stored in and read from the local repository's `.git/config`.

### 5.1. `.git/config`
Twinkle currently has a few settings that can be changed using `git config`:

```sh
git config sync.pollingInterval 3m  # (s)ec / (m)in / (h)our
git config sync.lfs.enabled true
git config sync.lfs.sizeThreshold 3m
```

### 5.2. `.twinkle/config`

Twinkle automatically adds `.twinkle/config` to the repository. This file contains a ***random id*** essential for Twinkle to function and will be recreated when missing:

```yml
[sync]
    id = abf70479d5283900df3f4765fc4083801b46e8d41355136f778ede6812413f67
```

<br>

## 6. Push Notifications

Twinkle uses polling to detect new remote commits. This is relatively resource-intensive and might trigger host rate limits, so can't be done as often. Syncing from the remote may be delayed by a minute or two.

To sync faster, [**twinkle-pushd**](https://github.com/hbons/twinkle-pushd) exists. Using [Secure WebSockets (wss://)](https://developer.mozilla.org/docs/Glossary/WebSockets) it immediately alerts all clients to start syncing. This feature is ***optional*** and syncing will work just fine without it.

Use `push.sparkleshare,org` or [host your own](https://github.com/hbons/twinkle-pushd) and set up your repositories like this:

```bash
git config --file=.twinkle/config sync.push.enabled true
git config --file=.twinkle/config sync.push.url wss://push.sparkleshare.org:443
```

These settings will sync to everyone connected to the repository through the `.twinkle/config` file. No sensitive data is transmitted. Only a hash of `sync.id`.

<br>

## 7. Links

* Fediverse ～ [@hbons@mastodon.social](https://mastodon.social/@hbons)
* Join the community on Matrix ～ [#sparkleshare:matrix.org](https://matrix.to/#/#sparkleshare:matrix.org)
* Website ～ [sparkleshare.org](https://sparkleshare.org)
