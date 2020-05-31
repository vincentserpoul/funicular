use funicular::apk_overlay::APKOverlay;

#[test]
fn alloptions_noprovisioner() {
    let overlay = APKOverlay::from_path("./tests/configs/no_provisioner.yaml").unwrap();
    assert_eq!(overlay.base.hostname, String::from("no_provisioner"));
}

#[test]
fn with_provisioners() {
    let overlay = APKOverlay::from_path("./tests/configs/with_provisioners.yaml").unwrap();
    assert_eq!(overlay.base.hostname, String::from("with_provisioners"));
}
