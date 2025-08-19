//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::ssh::keyscan::ssh_keyscan;
use crate::ssh::keys::key_type::KeyType;


#[test]
fn test_ssh_keyscan_github() {
    let host = "github.com";
    let port = 22;

    let github_rsa_public_key = "AAAAB3NzaC1yc2EAAAADAQABAAABgQCj7ndNxQowgcQnjshcLrqPEiiphnt+VTTvDP6mHBL9j1aNUkY4Ue1gvwnGLVlOhGeYrnZaMgRK6+PKCUXaDbC7qtbW8gIkhL7aGCsOr/C56SJMy/BCZfxd1nWzAOxSDPgVsmerOBYfNqltV9/hWCqBywINIR+5dIg6JTJ72pcEpEjcYgXkE2YEFXV1JHnsKgbLWNlhScqb2UmyRkQyytRLtL+38TGxkxCflmO+5Z8CSSNY7GidjMIZ7Q4zMjA2n1nGrlTDkzwDCsw+wqFPGQA179cnfGWOWRVruj16z6XyvxvjJwbz0wQZ75XK5tKSb7FNyeIEs4TT4jk+S4dhPeAUC5y+bDYirYgM4GC7uEnztnZyaVWQ7B381AK4Qdrwt51ZqExKbQpTUNn+EjqoTwvqNj4kqx5QUCI0ThS/YkOxJCXmPUWZbhjpCg56i+2aB6CmK2JGhn57K5mj0MNdBXA4/WnwH6XoPWJzK5Nyu2zB3nAZp+S5hpQs+p1vN1/wsjk=";
    let github_ecdsa_public_key = "AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBEmKSENjQEezOmxkZMy7opKgwFB9nkt5YRrYMjNuG5N87uRgg6CLrbo5wAdT/y6v0mKV0U2w0WZ2YB/++Tpockg=";
    let github_ed25519_public_key = "AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl";

    let host_key = ssh_keyscan(host, Some(port), KeyType::RSA).unwrap();
    assert_eq!(host_key.host, host);
    assert_eq!(host_key.key_type, KeyType::RSA);
    assert_eq!(host_key.public_key, github_rsa_public_key);

    let host_key = ssh_keyscan(host, Some(port), KeyType::ECDSA).unwrap();
    assert_eq!(host_key.host, host);
    assert_eq!(host_key.key_type, KeyType::ECDSA);
    assert_eq!(host_key.public_key, github_ecdsa_public_key);

    let host_key = ssh_keyscan(host, Some(port), KeyType::ED25519).unwrap();
    assert_eq!(host_key.host, host);
    assert_eq!(host_key.key_type, KeyType::ED25519);
    assert_eq!(host_key.public_key, github_ed25519_public_key);
}


#[test]
fn test_ssh_keyscan_gitlab() {
    let host = "gitlab.com";
    let port = 22;

    let gitlab_rsa_public_key = "AAAAB3NzaC1yc2EAAAADAQABAAABAQCsj2bNKTBSpIYDEGk9KxsGh3mySTRgMtXL583qmBpzeQ+jqCMRgBqB98u3z++J1sKlXHWfM9dyhSevkMwSbhoR8XIq/U0tCNyokEi/ueaBMCvbcTHhO7FcwzY92WK4Yt0aGROY5qX2UKSeOvuP4D6TPqKF1onrSzH9bx9XUf2lEdWT/ia1NEKjunUqu1xOB/StKDHMoX4/OKyIzuS0q/T1zOATthvasJFoPrAjkohTyaDUz2LN5JoH839hViyEG82yB+MjcFV5MU3N1l1QL3cVUCh93xSaua1N85qivl+siMkPGbO5xR/En4iEY6K2XPASUEMaieWVNTRCtJ4S8H+9";
    let gitlab_ecdsa_public_key = "AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBFSMqzJeV9rUzU4kWitGjeR4PWSa29SPqJ1fVkhtj3Hw9xjLVXVYrU9QlYWrOLXBpQ6KWjbjTDTdDkoohFzgbEY=";
    let gitlab_ed25519_public_key = "AAAAC3NzaC1lZDI1NTE5AAAAIAfuCHKVTjquxvt6CM6tdG4SLp1Btn/nOeHHE5UOzRdf";

    let host_key = ssh_keyscan(host, Some(port), KeyType::RSA).unwrap();
    assert_eq!(host_key.public_key, gitlab_rsa_public_key);

    let host_key = ssh_keyscan(host, Some(port), KeyType::ECDSA).unwrap();
    assert_eq!(host_key.public_key, gitlab_ecdsa_public_key);

    let host_key = ssh_keyscan(host, Some(port), KeyType::ED25519).unwrap();
    assert_eq!(host_key.public_key, gitlab_ed25519_public_key);
}
