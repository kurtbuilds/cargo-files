fn run_test(krate: &tempfile::TempDir) -> String {
    let crate_root = dunce::canonicalize(krate.path()).unwrap();
    let projects =
        cargo_files_core::get_targets(Some(&crate_root.join("Cargo.toml")), None).unwrap();

    let mut paths = Vec::new();
    for target in projects {
        let files = cargo_files_core::get_target_files(&target).unwrap();
        for file in files {
            let relative_path = pathdiff::diff_paths(&file, &crate_root).unwrap();
            let components: Vec<_> = relative_path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            paths.push(components.join("/"));
        }
    }
    paths.sort();
    paths.join("\n")
}

/// Generate a test case which detects which files are present in a crate.
macro_rules! krate {
    ($def:literal) => {
        let krate = ::cargo_files_test::make_crate!($def);
        insta::assert_snapshot!(run_test(&krate));
    };
}

#[test]
fn basic_detection() {
    krate!(
        "
        src:
          - lib.rs [test, whatever]
          - whatever.rs
          - test:
            - mod.rs [cat]
            - cat.rs
            - not_in_the_crate.rs
    "
    );
}

#[test]
fn new_module_layout() {
    krate!(
        "
        src:
          - lib.rs [scooby]
          - scooby.rs [apple, banana]
          - scooby:
            - apple.rs
            - banana.rs
    "
    );
}

#[test]
fn path_attribute() {
    krate!(
        r#"
        src:
          - lib.rs [scooby(apple.rs), banana]
          - apple.rs
          - banana:
            - mod.rs
    "#
    );
}
