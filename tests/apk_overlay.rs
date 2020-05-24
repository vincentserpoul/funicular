use funicular::apk_overlay::APKOverlay;

#[test]
fn alloptions_noprovisioner() {
    let overlay = APKOverlay::from_path("./tests/configs/alloptions_noprovisioner.yaml").unwrap();
    assert_eq!(
        overlay.base.hostname,
        String::from("alloptions_noprovisioner")
    );
}

#[test]
fn nooption_noprovisioner() {
    let overlay = APKOverlay::from_path("./tests/configs/nooption_noprovisioner.yaml").unwrap();
    assert_eq!(
        overlay.base.hostname,
        String::from("nooption_noprovisioner")
    );
}
