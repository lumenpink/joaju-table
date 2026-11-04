#[test]
fn test_rust_version_constant() {
    assert!(
        !joaju_table::VERSION.is_empty(),
        "VERSION const cannot be empty"
    );
    assert!(
        joaju_table::VERSION.starts_with("0.") || joaju_table::VERSION.starts_with("1."),
        "VERSION should start with SemVer major digit"
    );
}

#[test]
fn test_ci_workflow_kombinat_notification() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        !ci_yaml.contains("codeberg.org"),
        "Codeberg dispatch must be completely removed"
    );
    assert!(
        ci_yaml.contains("http://127.0.0.1:4040/api/cascade/dispatch"),
        "CI workflow must notify Kombinat cascade daemon on release"
    );
}

#[test]
fn test_ci_workflow_no_duplicate_registries() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        !ci_yaml.contains("[registries.lumen]"),
        "CI workflow must not configure duplicate lumen registry"
    );
    assert!(
        ci_yaml.contains("cargo publish --registry gambi"),
        "CI workflow must publish to registry gambi"
    );
    assert!(
        ci_yaml.contains("http://172.17.0.1:4040/api/cascade/dispatch"),
        "CI workflow must include docker bridge fallback for Kombinat"
    );
    assert!(
        !ci_yaml.contains("Sovereign"),
        "CI workflow must purge imperialist terminology"
    );
    assert!(
        ci_yaml.contains("Publish Canonical Crate"),
        "Workflow job name must be Canonical"
    );
}
