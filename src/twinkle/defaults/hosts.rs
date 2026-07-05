//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub fn twinkle_host_ssh_settings_url(host: String) -> Option<&'static str> {
    match host.as_str() { // TODO: Use an enum to cover all cases
        "bitbucket.org"        => Some("https://bitbucket.org/account/settings/ssh-keys/"),
        "codeberg.org"         => Some("https://codeberg.org/user/settings/keys"),
        "git.sr.ht"            => Some("https://meta.sr.ht/keys/ssh-keys"),
        "git.code.sf.net"      => None, // TODO: Don't have a SourceForge account
        "git.launchpad.net"    => Some("https://launchpad.net/~/+editsshkeys"),
        "git.savannah.gnu.org" => Some("https://savannah.gnu.org/my/admin/editsshkeys.php"),
        "gitee.com"            => Some("https://gitee.com/profile/sshkeys"),
        "github.com"           => Some("https://github.com/settings/keys"),
        "gitlab.com"           => Some("https://gitlab.com/-/user_settings/ssh_keys"),
        "gitlab.gnome.org"     => Some("https://gitlab.gnome.org/-/user_settings/ssh_keys"),
        "invent.kde.org"       => Some("https://invent.kde.org/-/user_settings/ssh_keys"),
        "ssh.dev.azure.com"    => None, // No universal link available
        _ => None
    }
}
