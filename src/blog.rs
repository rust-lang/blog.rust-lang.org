#[path = "lib.rs"]
mod lib;

pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    lib::main()?;

    println!(
        "blog has been generated; you can now serve its content by running\n\
              {INDENT}python3 -m http.server --directory {ROOT}/site\n\
              or running:\n\
              {INDENT}cargo run -p serve\n\
              or you can read it directly by opening a web browser on:\n\
              {INDENT}file:///{ROOT}/site/index.html",
        ROOT = env!("CARGO_MANIFEST_DIR"),
        INDENT = "    "
    );

    Ok(())
}
