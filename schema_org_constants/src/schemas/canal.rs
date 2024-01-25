/// <https://schema.org/Canal>
pub const CANAL_IRI_HTTP: &str = "http://schema.org/Canal";
/// <https://schema.org/Canal>
pub const CANAL_IRI_HTTPS: &str = "https://schema.org/Canal";
/// <https://schema.org/Canal>
pub const CANAL_LABEL: &str = "Canal";
pub struct CanalIri;
impl PartialEq<&str> for CanalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CANAL_IRI_HTTP || *other == CANAL_IRI_HTTPS
	}
}
impl PartialEq<CanalIri> for &str {
	fn eq(&self, other: &CanalIri) -> bool {
		*self == CANAL_IRI_HTTP || *self == CANAL_IRI_HTTPS
	}
}
pub struct CanalIriOrLabel;
impl PartialEq<&str> for CanalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CanalIri || *other == CANAL_LABEL
	}
}
impl PartialEq<CanalIriOrLabel> for &str {
	fn eq(&self, other: &CanalIriOrLabel) -> bool {
		*self == CanalIri || *self == CANAL_LABEL
	}
}
