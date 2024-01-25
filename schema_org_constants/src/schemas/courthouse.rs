/// <https://schema.org/Courthouse>
pub const COURTHOUSE_IRI_HTTP: &str = "http://schema.org/Courthouse";
/// <https://schema.org/Courthouse>
pub const COURTHOUSE_IRI_HTTPS: &str = "https://schema.org/Courthouse";
/// <https://schema.org/Courthouse>
pub const COURTHOUSE_LABEL: &str = "Courthouse";
pub struct CourthouseIri;
impl PartialEq<&str> for CourthouseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURTHOUSE_IRI_HTTP || *other == COURTHOUSE_IRI_HTTPS
	}
}
impl PartialEq<CourthouseIri> for &str {
	fn eq(&self, other: &CourthouseIri) -> bool {
		*self == COURTHOUSE_IRI_HTTP || *self == COURTHOUSE_IRI_HTTPS
	}
}
pub struct CourthouseIriOrLabel;
impl PartialEq<&str> for CourthouseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourthouseIri || *other == COURTHOUSE_LABEL
	}
}
impl PartialEq<CourthouseIriOrLabel> for &str {
	fn eq(&self, other: &CourthouseIriOrLabel) -> bool {
		*self == CourthouseIri || *self == COURTHOUSE_LABEL
	}
}
