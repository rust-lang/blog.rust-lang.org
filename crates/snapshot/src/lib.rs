#[test]
fn snapshot() {
    std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).unwrap();
    let _ = std::fs::remove_dir_all("public");
    let status = std::process::Command::new("cargo")
        .args(["zola", "build"])
        .status()
        .unwrap();
    assert!(status.success(), "failed to build site");

    let timestamped_files = ["releases.json", "feed.xml"];
    let inexplicably_non_deterministic_files =
        ["2023/08/07/Rust-Survey-2023-Results/experiences.png"];
    insta::glob!("../../..", "public/**/*", |path| {
        if path.is_dir() {
            return;
        }
        let path = path.display().to_string();
        if timestamped_files
            .into_iter()
            .chain(inexplicably_non_deterministic_files)
            .any(|f| path.ends_with(f))
        {
            // Skip troublesome files, e.g. they might contain timestamps.
            // If possible, they are tested separately below.
            return;
        }
        let content = std::fs::read(path).unwrap();
        // insta can't deal with non-utf8 strings?
        let content = String::from_utf8_lossy(&content).into_owned();
        insta::assert_snapshot!(content);
    });

    // test files with timestamps filtered
    insta::with_settings!({filters => vec![
        (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}", "(filtered timestamp)"),
    ]}, {
        for file in timestamped_files {
            let content = std::fs::read(format!("public/{file}")).unwrap();
            let content = String::from_utf8_lossy(&content).into_owned();
            insta::assert_snapshot!(content);
        }
    });
}
