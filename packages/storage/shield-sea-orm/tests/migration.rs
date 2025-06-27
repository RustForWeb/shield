use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn example_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../examples/sea-orm")
}

const BACKENDS: &[(&str, &str)] = &[
    ("mysql", "mysql://shield:shield@localhost:13306/shield"),
    (
        "postgresql",
        "postgres://shield:shield@localhost:15432/shield",
    ),
    ("sqlite", "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc"),
];

#[test]
pub fn migrations() {
    for (backend, url) in BACKENDS {
        // Check up migrations
        assert!(
            Command::new("sea-orm-cli")
                .arg("migrate")
                .arg("fresh")
                .arg("-u")
                .arg(url)
                .arg("-d")
                .arg(example_path())
                .status()
                .unwrap_or_else(|_| panic!("{backend} up migrations should succeed."))
                .success()
        );

        // Check down migrations
        assert!(
            Command::new("sea-orm-cli")
                .arg("migrate")
                .arg("refresh")
                .arg("-u")
                .arg(url)
                .arg("-d")
                .arg(example_path())
                .status()
                .unwrap_or_else(|_| panic!("{backend} down migrations should succeed."))
                .success()
        );

        // Cleanup
        assert!(
            Command::new("sea-orm-cli")
                .arg("migrate")
                .arg("reset")
                .arg("-u")
                .arg(url)
                .arg("-d")
                .arg(example_path())
                .status()
                .unwrap_or_else(|_| panic!("{backend} cleanup should succeed."))
                .success()
        );
    }
}
