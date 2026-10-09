#[test]
fn snapshot() {
    std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).unwrap();
    let _ = std::fs::remove_dir_all("public");
    let status = std::process::Command::new("zola")
        .arg("build")
        .status()
        .unwrap();
    assert!(status.success(), "failed to build site");

    let timestamped_files = ["releases.json", "feed.xml"];
    insta::glob!("../../..", "public/**/*", |path| {
        if path.is_dir() {
            return;
        }
        let content = std::fs::read(path).unwrap();
        let Ok(content) = std::str::from_utf8(&content) else {
            let extension = path.extension().unwrap().to_str().unwrap();
            insta::assert_binary_snapshot!(&format!(".{extension}"), content);
            return;
        };

        let path = path.display().to_string();
        if timestamped_files.into_iter().any(|f| path.ends_with(f)) {
            insta::with_settings!({filters => vec![
                (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}", "(filtered timestamp)"),
            ]}, {
                insta::assert_snapshot!(content);
            });
        } else {
            insta::assert_snapshot!(content);
        }
    });
}
