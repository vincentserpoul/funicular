use funicular::config::apk_overlay::base::Arch;
use funicular::config::apk_overlay::APKOverlay;

#[test]
fn alloptions_noprovisioner() {
    let overlay =
        APKOverlay::from_path("./tests/config/apk_overlay/configs/no_provisioner.toml").unwrap();
    assert_eq!(overlay.base.hostname, String::from("no_provisioner"));
    assert_eq!(overlay.base.arch, Arch::ARMHF);
}

#[test]
fn with_provisioners() {
    let overlay =
        APKOverlay::from_path("./tests/config/apk_overlay/configs/with_provisioners.toml").unwrap();
    assert_eq!(overlay.base.hostname, String::from("with_provisioners"));
    assert_eq!(overlay.base.arch, Arch::AARCH64);

    let provisioners = overlay.provisioners.unwrap();
    assert_eq!(provisioners[1].name, String::from("two_factor_auth"));
    assert_eq!(
        *provisioners[1].environment_vars.0.get("code").unwrap(),
        String::from("ADCGGDI")
    );
}

#[test]
fn provisioners_to_env_vars() {
    let overlay =
        APKOverlay::from_path("./tests/config/apk_overlay/configs/with_provisioners.toml").unwrap();
    let env_vars = overlay.to_hash_map();
    assert_eq!(
        env_vars.contains_key("PROVISIONER_TWO_FACTOR_AUTH_CODE"),
        true
    );
    assert_eq!(
        *env_vars.get("PROVISIONERS").unwrap(),
        String::from("wlan two_factor_auth k3s")
    );
    assert_eq!(
        *env_vars.get("PROVISIONER_TWO_FACTOR_AUTH_CODE").unwrap(),
        String::from("ADCGGDI")
    );
}

#[test]
fn provisioners_to_string() {
    let overlay =
        APKOverlay::from_path("./tests/config/apk_overlay/configs/with_provisioners.toml").unwrap();
    let overlay = overlay.to_string();

    let test_env_vars = [
        r#"BASE_USERS_REMOTE_USER="funi"#,
        r#"BASE_HOSTNAME="with_provisioners"#,
        r#"BASE_ARCH="aarch64"#,
        r#"BASE_SSH_AUTHORIZED_KEYS="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMIVp6q5co/r5GwY0dH+NYQbfKicapeF3gXEU3dzaAvD me@home, ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCiChinH9volauTvLfGWv2xCIo0jrQAv0jCQjfDodZW+E1vLFUcgdULKemujxG2vLzLUHfSHF9mjnwnGbyHYZi1fEO70s3gGZNd9K2xwvkGo28svefCfNR3hi+jSB9Q9drvR7CgYdEY5D90Z/OfSWJ4a60/qpD7L3uXf5riqYddDUbHVlDg11SK27KHan33UAfskd5u2AccRbXKJX3I6oO78AwI4/fHs2N/RuoleYcsHX9FNaVX8NHxSEY7EXLTPmykRQj8/8ubjuflvm4qYTsW8cFtRETfxkgFMF0p375YEVQles/6JwRsljnVaobiyeNG1u/5p4zaEguuqN7oVpsP me@home"#,
        r#"BASE_ALPINE_MIRROR="http://dl-cdn.alpinelinux.org/alpine"#,
        r#"BASE_ALPINE_VERSION="3.12.0"#,
        r#"BASE_ALPINE_TIMEZONE="Asia/Singapore"#,
        r#"BASE_USERS_REMOTE_USER_PASSWORD="funipass"#,
        r#"BASE_USERS_ROOT_PASSWORD="rootpass"#,
        r#"BASE_NETWORKING_DNS_NAMESERVERS="8.8.8.8, 1.1.1.1"#,
        r#"PROVISIONERS="wlan two_factor_auth k3s"#,
    ];

    test_env_vars.iter().for_each(|s| {
        assert_eq!(
            overlay.contains(*s),
            true,
            "should contain {}, but did not\n{}",
            *s,
            overlay
        );
    });
}

#[test]
fn default_conf() {
    let overlay = APKOverlay::from_path("./tests/config/apk_overlay/configs/default.toml").unwrap();
    let overlay = overlay.to_string();

    let test_env_vars = [
        r#"BASE_ARCH="aarch64"#,
        r#"BASE_ALPINE_MIRROR="http://dl-cdn.alpinelinux.org/alpine"#,
        r#"BASE_ALPINE_VERSION="3.12.0"#,
        r#"BASE_ALPINE_TIMEZONE="Asia/Singapore"#,
        r#"BASE_NETWORKING_DNS_NAMESERVERS="1.1.1.1, 8.8.8.8"#,
    ];

    test_env_vars.iter().for_each(|s| {
        assert_eq!(
            overlay.contains(*s),
            true,
            "should contain {}, but did not",
            *s
        );
    });
}
