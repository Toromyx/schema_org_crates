/// <https://schema.org/Atlas>
pub const ATLAS_IRI_HTTP: &str = "http://schema.org/Atlas";
/// <https://schema.org/Atlas>
pub const ATLAS_IRI_HTTPS: &str = "https://schema.org/Atlas";
/// <https://schema.org/Atlas>
pub const ATLAS_LABEL: &str = "Atlas";
pub struct AtlasIri;
impl PartialEq<&str> for AtlasIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ATLAS_IRI_HTTP || *other == ATLAS_IRI_HTTPS
	}
}
impl PartialEq<AtlasIri> for &str {
	fn eq(&self, other: &AtlasIri) -> bool {
		*self == ATLAS_IRI_HTTP || *self == ATLAS_IRI_HTTPS
	}
}
pub struct AtlasIriOrLabel;
impl PartialEq<&str> for AtlasIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AtlasIri || *other == ATLAS_LABEL
	}
}
impl PartialEq<AtlasIriOrLabel> for &str {
	fn eq(&self, other: &AtlasIriOrLabel) -> bool {
		*self == AtlasIri || *self == ATLAS_LABEL
	}
}
