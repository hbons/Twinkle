![The SparkleShare app icon](data/icons/hicolor/scalable/apps/studio.planetpeanut.SparkleShare.svg)

# [SparkleShare](https://sparkleshare.org) ～ *Sync files with Git*

The command-line version of [SparkleShare](https://sparkleshare.org). Sync your files with any Git repository. Collaborate using services like [GitHub](https://github.com), [GitLab](https://gitlab.com) and [Codeberg](https://codeberg.org) or host your own using [Forgejo](https://forgejo.org). No special server software required and its only dependencies are `openssh`, `git`, and (optionally) `git-lfs`.

<br>

[![Flatpak](https://github.com/hbons/Twinkle/actions/workflows/integration.yml/badge.svg)](https://github.com/hbons/Twinkle/actions/workflows/integration.yml)

<br>


## 1. Install on Linux / macOS

```shell
cargo install --path .
```

Or get the [nightly Flatpak](https://github.com/hbons/SparkleShare/releases/tag/flatpak-nightly).

```shell
Usage: sparkleshare <command> [args…]

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
> ***SparkleShare is still in preview***. Data loss is unlikely but possible. Run commands with `DEBUG=1 sparkleshare <COMMAND>` to see what's going on under the hood and run `sparkleshare check` to diagnose setup problems. Use `git reflog` to inspect lost commits.


### 2.1. Starting with a remote repository

```sh
sparkleshare clone ssh://git@codeberg.org/user/repo

cd repo
git config --local user.name "Your Name"
git config --local user.email "your@email"

sparkleshare sync
```

### 2.2. Starting with local files

```sh
sparkleshare init ssh://git@codeberg.org/user/repo

git config --local user.name "Your Name"
git config --local user.email "your@email"

sparkleshare sync
```

<br>


## 3. Git LFS

SparkleShare supports the [Large File Storage](https://git-lfs.com) (LFS) extension. When a file larger than `size_threshold` is added, `git lfs track <FILE>` is called automatically. To enable LFS:

```sh
git lfs --version  # Check if git-lfs is installed

git config sync.lfs.enabled true
git config sync.lfs.sizeThreshold 3m  # (k)b / (m)b / (g)b

sparkleshare sync
```

In addition to the threshold, you can still add your own patterns to `.gitattributes`:

```sh
git lfs track *.mp4
```

<br>


## 4. Merge Strategy

> [!IMPORTANT]
> SparkleShare keeps a ***linear commit history*** by using `git-rebase` to merge commits.

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
> For predictability and portability reasons, SparkleShare
> ***ignores the global and system Git configuration files***.
> Settings are stored in and read from the local repository's `.git/config`.

### 5.1. `.git/config`
SparkleShare currently has a few settings that can be changed using `git config`:

```sh
git config sync.pollingInterval 3m  # (s)ec / (m)in / (h)our
git config sync.lfs.enabled true
git config sync.lfs.sizeThreshold 3m
```

### 5.2. `.sparkleshare/config`

SparkleShare automatically adds `.sparkleshare/config` to the repository. This file contains a ***random id*** essential for SparkleShare to function and will be recreated when missing:

```yml
[sync]
    id = abf70479d5283900df3f4765fc4083801b46e8d41355136f778ede6812413f67
```

<br>

## 6. Push Notifications

SparkleShare uses polling to detect new remote commits. This is relatively resource-intensive and might trigger host rate limits, so can't be done as often. Syncing from the remote may be delayed by a minute or two.

To sync faster, [**sparkleshare-pushd**](https://github.com/hbons/sparkleshare-pushd) exists. Using [Secure WebSockets (wss://)](https://developer.mozilla.org/docs/Glossary/WebSockets) it immediately alerts all clients to start syncing. This feature is ***optional*** and syncing will work just fine without it.

Use `push.sparkleshare,org` or [host your own](https://github.com/hbons/sparkleshare-pushd) and set up your repositories like this:

```bash
git config --file=.sparkleshare/config sync.push.enabled true
git config --file=.sparkleshare/config sync.push.url wss://push.sparkleshare.org:443
```

These settings will sync to everyone connected to the repository through the `.sparkleshare/config` file. No sensitive data is transmitted. Only a hash of `sync.id`.

<br>

## 7. Links

* Fediverse ～ [@hbons@mastodon.social](https://mastodon.social/@hbons)
* Join the community on Matrix ～ [#sparkleshare:matrix.org](https://matrix.to/#/#sparkleshare:matrix.org)
* Website ～ [sparkleshare.org](https://sparkleshare.org)
