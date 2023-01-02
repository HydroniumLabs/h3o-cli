use std::{env, path::PathBuf};

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

// From assert_cmd
fn target_dir() -> PathBuf {
    // Logic from `assert_cmd` crate.
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

/// Look up the path to a cargo-built binary within an integration test.
fn cargo_bin(name: &str) -> String {
    // Logic from `assert_cmd` crate.
    let env_var = format!("CARGO_BIN_EXE_{name}");
    std::env::var_os(env_var)
        .unwrap_or_else(|| {
            target_dir()
                .join(format!("{name}{}", env::consts::EXE_SUFFIX))
                .into_os_string()
        })
        .into_string()
        .expect("valid unicode path")
}

#[test]
fn lit() {
    lit::run::tests(lit::event_handler::Default::default(), |config| {
        config.add_search_path(format!("{CRATE_PATH}/tests/lit"));
        config.add_extension("geojson");
        config.add_extension("kml");
        config.add_extension("txt");
        config
            .constants
            .insert("cli".to_owned(), cargo_bin("h3o-cli"));
    })
    .expect("lit tests failed");
}
