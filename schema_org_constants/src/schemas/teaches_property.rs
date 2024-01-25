/// <https://schema.org/teaches>
pub const TEACHES_PROPERTY_IRI_HTTP: &str = "http://schema.org/teaches";
/// <https://schema.org/teaches>
pub const TEACHES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/teaches";
/// <https://schema.org/teaches>
pub const TEACHES_PROPERTY_LABEL: &str = "teaches";
pub struct TeachesPropertyIri;
impl PartialEq<&str> for TeachesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEACHES_PROPERTY_IRI_HTTP || *other == TEACHES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TeachesPropertyIri> for &str {
	fn eq(&self, other: &TeachesPropertyIri) -> bool {
		*self == TEACHES_PROPERTY_IRI_HTTP || *self == TEACHES_PROPERTY_IRI_HTTPS
	}
}
pub struct TeachesPropertyIriOrLabel;
impl PartialEq<&str> for TeachesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TeachesPropertyIri || *other == TEACHES_PROPERTY_LABEL
	}
}
impl PartialEq<TeachesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TeachesPropertyIriOrLabel) -> bool {
		*self == TeachesPropertyIri || *self == TEACHES_PROPERTY_LABEL
	}
}
