![The SparkleShare app icon](data/icons/scalable/studio.planetpeanut.Twinkle.svg)

# [Twinkle](https://sparkleshare.org) ～ *Sync files with Git*

The command-line version of [SparkleShare](https://sparkleshare.org). Sync your files with any Git repository. Collaborate using services like GitHub, GitLab and Codeberg or host your own. No special server software required and its only dependencies are `openssh`, `git`, and (optionally) `git-lfs`.

<br>


## 1. Install on Linux / macOS

```shell
cargo install --path .
```

[![Flatpak](https://github.com/hbons/Twinkle/actions/workflows/flatpak.yml/badge.svg)](https://github.com/hbons/Twinkle/actions/workflows/flatpak.yml)

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
> ***Twinkle is still in preview***. Data loss is unlikely but possible. Run commands with `DEBUG=1 twinkle <COMMAND>` to see what's going on under the hood.


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

git config twinkle.lfs.enabled true
git config twinkle.lfs.sizeThreshold 3m  # (k)b / (m)b / (g)b

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

Here's how it works when *Alice* and *Bob* both make changes to `README.md` at the same time:

* Alice adds, Bob adds:
  * `README (Alice).md` and `README (Bob).md`
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
git config twinkle.pollingInterval 3m  # (s)ec / (m)in / (h)our
git config twinkle.lfs.enabled true
git config twinkle.lfs.sizeThreshold 3m
```

### 5.2. `.twinkle.conf`

Twinkle automatically adds `.twinkle.conf` to the repository. This file contains a ***random id*** essential for Twinkle to function and will be recreated when missing:

```yml
[twinkle]
    id = abf70479d5283900df3f4765fc4083801b46e8d41355136f778ede6812413f67
```

<br>


## 6. Links

* [Hylke on Mastodon](https://mastodon.social/@hbons)
* [planetpeanut.studio](https://planetpeanut.studio)
* [sparkleshare.org](https://sparkleshare.org)
