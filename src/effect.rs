use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectInfo {
    description: String,
    source_urls: Vec<String>,
    effects: Vec<Effect>,
}

type BacResult<T> = Result<T, std::io::Error>;

impl EffectInfo {
    pub fn from_path(path: impl Into<PathBuf>) -> BacResult<EffectInfo> {
        let path = path.into();
        let mut f = File::open(&path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        EffectInfo::from_str(&buf)
    }

    pub fn from_str(s: &str) -> BacResult<EffectInfo> {
        let info: EffectInfo = serde_json::from_str(s)?;
        Ok(info)
    }

    pub fn get_effect(&self, bac: f64) -> Option<&Effect> {
        self.effects.iter().find(|effect| effect.in_range(bac))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    range: Range<f64>,
    behaviors: Vec<String>,
    impairments: Vec<String>,
}

impl Effect {
    fn in_range(&self, n: f64) -> bool {
        self.range.contains(&n)
    }

    pub fn behaviors(&self) -> String {
        self.behaviors
            .iter()
            .map(|b| format!("- {}\n", b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_from_path() {
        let eff_info = EffectInfo::from_path("data/effects.json").unwrap();
        assert!(!eff_info.description.is_empty());
    }

    #[test]
    fn can_get_an_effect() {
        let eff_info = EffectInfo::from_path("data/effects.json").unwrap();
        let effect = eff_info.get_effect(0.2).unwrap();
        assert!(!effect.behaviors.is_empty());
    }
}
