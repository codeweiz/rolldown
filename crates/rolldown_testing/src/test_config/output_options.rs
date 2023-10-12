use crate::impl_serde_default;
use rustc_hash::FxHashMap;
use schemars::JsonSchema;
use serde::Deserialize;

fn esm_by_default() -> String {
  "esm".to_string()
}

fn auto_by_default() -> String {
  "auto".to_string()
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutputOptions {
  #[serde(default = "esm_by_default")]
  pub format: String,
  #[serde(default = "auto_by_default")]
  pub export_mode: String,
  pub manual_chunks: Option<FxHashMap<String, Vec<String>>>,
}

impl_serde_default!(OutputOptions);
