use funicular::apk_overlay::env_vars::EnvVars;
use funicular::apk_overlay::APKOverlay;

#[test]
fn alloptions_noprovisioner() {
    let overlay = APKOverlay::from_path(
        "./tests/apk_overlay/configs/no_provisioner.yaml",
    )
    .unwrap();
    assert_eq!(overlay.base.hostname, String::from("no_provisioner"));
}

#[test]
fn with_provisioners() {
    let overlay = APKOverlay::from_path(
        "./tests/apk_overlay/configs/with_provisioners.yaml",
    )
    .unwrap();
    assert_eq!(overlay.base.hostname, String::from("with_provisioners"));

    let provisioners = overlay.provisioners.unwrap();
    assert_eq!(provisioners[1].name, String::from("two_factor_auth"));
    assert_eq!(
        *provisioners[1].environment_vars.0.get("code").unwrap(),
        String::from("ADCGGDI")
    );
}

#[test]
fn provisioners_to_env_vars() {
    let overlay = APKOverlay::from_path(
        "./tests/apk_overlay/configs/with_provisioners.yaml",
    )
    .unwrap();
    let env_vars = overlay.to_hash_map("");
    println!("{:?}", env_vars);
    assert_eq!(
        env_vars.contains_key("PROVISIONER_TWO_FACTOR_AUTH_CODE"),
        true
    );
    assert_eq!(
        *env_vars.get("PROVISIONER_TWO_FACTOR_AUTH_CODE").unwrap(),
        String::from("ADCGGDI")
    );
}
