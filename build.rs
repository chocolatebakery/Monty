#[cfg(not(feature = "nonet"))]
use std::env;

#[cfg(not(feature = "nonet"))]
const DEFAULT_VALUE_PATH: &str = "resources/value.network";

#[cfg(not(feature = "nonet"))]
const DEFAULT_POLICY_PATH: &str = "resources/policy.network";

#[cfg(not(feature = "nonet"))]
fn main() {
    println!("cargo:rerun-if-env-changed=EVALFILE");
    println!("cargo:rerun-if-changed=resources/value.network");

    let value_path = env::var("EVALFILE").unwrap_or(DEFAULT_VALUE_PATH.into());
    if value_path != DEFAULT_VALUE_PATH {
        std::fs::copy(value_path, DEFAULT_VALUE_PATH).unwrap();
    }

    println!("cargo:rerun-if-env-changed=POLICYFILE");
    println!("cargo:rerun-if-changed=resources/policy.network");

    let policy_path = env::var("POLICYFILE").unwrap_or(DEFAULT_POLICY_PATH.into());
    if policy_path != DEFAULT_POLICY_PATH {
        std::fs::copy(policy_path, DEFAULT_POLICY_PATH).unwrap();
    }
}

#[cfg(feature = "nonet")]
fn main() {}
