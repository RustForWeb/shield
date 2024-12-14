use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn example_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../examples/seaorm")
}

#[test]
pub fn mysql() {
    assert!(Command::new("sea-orm-cli")
        .arg("migrate")
        .arg("fresh")
        .arg("-u")
        .arg("mysql://shield:shield@localhost:13306/shield")
        .arg("-d")
        .arg(example_path())
        .status()
        .expect("MySQL migration should succeed.")
        .success());
}

#[test]
pub fn postgresql() {
    assert!(Command::new("sea-orm-cli")
        .arg("migrate")
        .arg("fresh")
        .arg("-u")
        .arg("postgres://shield:shield@localhost:15432/shield")
        .arg("-d")
        .arg(example_path())
        .status()
        .expect("MySQL migration should succeed.")
        .success());
}

#[test]
pub fn sqlite() {
    assert!(Command::new("sea-orm-cli")
        .arg("migrate")
        .arg("fresh")
        .arg("-u")
        .arg("sqlite:///tmp/shield-seaorm.sqlite?mode=rwc")
        .arg("-d")
        .arg(example_path())
        .status()
        .expect("MySQL migration should succeed.")
        .success());
}
