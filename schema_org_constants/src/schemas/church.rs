/// <https://schema.org/Church>
pub const CHURCH_IRI_HTTP: &str = "http://schema.org/Church";
/// <https://schema.org/Church>
pub const CHURCH_IRI_HTTPS: &str = "https://schema.org/Church";
/// <https://schema.org/Church>
pub const CHURCH_LABEL: &str = "Church";
pub struct ChurchIri;
impl PartialEq<&str> for ChurchIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHURCH_IRI_HTTP || *other == CHURCH_IRI_HTTPS
	}
}
impl PartialEq<ChurchIri> for &str {
	fn eq(&self, other: &ChurchIri) -> bool {
		*self == CHURCH_IRI_HTTP || *self == CHURCH_IRI_HTTPS
	}
}
pub struct ChurchIriOrLabel;
impl PartialEq<&str> for ChurchIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChurchIri || *other == CHURCH_LABEL
	}
}
impl PartialEq<ChurchIriOrLabel> for &str {
	fn eq(&self, other: &ChurchIriOrLabel) -> bool {
		*self == ChurchIri || *self == CHURCH_LABEL
	}
}
