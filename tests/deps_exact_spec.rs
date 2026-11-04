#[test]
fn test_joaju_csv_dependency_has_exact_semver() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml must exist");
    assert!(
        cargo_toml.contains("joaju-csv = { version = \"=0.2.0-dev.44\", registry = \"gambi\" }"),
        "joaju-csv dependency must be exact (=0.2.0-dev.44) and pointed to gambi registry"
    );
}

#[test]
fn test_ci_workflow_has_segregated_dev_aliases() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        ci_yaml.contains("latest-dev"),
        "ci.yml must support latest-dev alias for dev channel"
    );
}
