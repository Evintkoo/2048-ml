//! Versioned input contract for root-level HyperOptX search settings.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

pub const HYPEROPT_CONFIG_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntegerRange {
    pub low: i64,
    pub high: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HyperOptSearchConfig {
    pub schema_version: u32,
    pub n_trials: usize,
    pub sampler: String,
    pub n_estimators: IntegerRange,
    pub max_depth: IntegerRange,
}

impl HyperOptSearchConfig {
    pub fn defaults(n_trials: usize) -> Self {
        Self {
            schema_version: HYPEROPT_CONFIG_SCHEMA_VERSION,
            n_trials,
            sampler: "tpe".to_string(),
            n_estimators: IntegerRange { low: 20, high: 200 },
            max_depth: IntegerRange { low: 2, high: 10 },
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.schema_version == HYPEROPT_CONFIG_SCHEMA_VERSION,
            "unsupported HyperOpt config schema version {}; supported version is {}",
            self.schema_version,
            HYPEROPT_CONFIG_SCHEMA_VERSION
        );
        anyhow::ensure!(self.n_trials > 0, "n_trials must be greater than zero");
        anyhow::ensure!(
            self.sampler.eq_ignore_ascii_case("tpe"),
            "sampler must be 'tpe' for the current root integration"
        );
        validate_range("n_estimators", &self.n_estimators)?;
        validate_range("max_depth", &self.max_depth)?;
        Ok(())
    }

    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path)?;
        let config: Self = serde_json::from_slice(&bytes)?;
        config.validate()?;
        Ok(config)
    }
}

fn validate_range(name: &str, range: &IntegerRange) -> anyhow::Result<()> {
    anyhow::ensure!(range.low > 0, "{name}.low must be greater than zero");
    anyhow::ensure!(range.high >= range.low, "{name}.high must be >= {name}.low");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_search_config_is_versioned_and_valid() {
        let config = HyperOptSearchConfig::defaults(7);
        config.validate().unwrap();
        assert_eq!(config.schema_version, HYPEROPT_CONFIG_SCHEMA_VERSION);
        assert_eq!(config.n_trials, 7);
    }

    #[test]
    fn search_config_rejects_unknown_versions_ranges_and_samplers() {
        let mut config = HyperOptSearchConfig::defaults(1);
        config.schema_version += 1;
        assert!(config.validate().is_err());

        let mut config = HyperOptSearchConfig::defaults(1);
        config.max_depth.high = 1;
        assert!(config.validate().is_err());

        let mut config = HyperOptSearchConfig::defaults(1);
        config.sampler = "grid".to_string();
        assert!(config.validate().is_err());
    }
}
