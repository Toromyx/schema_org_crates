// https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg
#![cfg_attr(doc, feature(doc_auto_cfg))]

#[cfg(any(feature = "json-ld_0_15", doc))]
pub mod json_ld_0_15;
mod schemas;
pub use self::schemas::*;
