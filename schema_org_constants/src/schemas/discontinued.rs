/// <https://schema.org/Discontinued>
pub const DISCONTINUED_IRI_HTTP: &str = "http://schema.org/Discontinued";
/// <https://schema.org/Discontinued>
pub const DISCONTINUED_IRI_HTTPS: &str = "https://schema.org/Discontinued";
/// <https://schema.org/Discontinued>
pub const DISCONTINUED_LABEL: &str = "Discontinued";
pub struct DiscontinuedIri;
impl PartialEq<&str> for DiscontinuedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCONTINUED_IRI_HTTP || *other == DISCONTINUED_IRI_HTTPS
	}
}
impl PartialEq<DiscontinuedIri> for &str {
	fn eq(&self, other: &DiscontinuedIri) -> bool {
		*self == DISCONTINUED_IRI_HTTP || *self == DISCONTINUED_IRI_HTTPS
	}
}
pub struct DiscontinuedIriOrLabel;
impl PartialEq<&str> for DiscontinuedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscontinuedIri || *other == DISCONTINUED_LABEL
	}
}
impl PartialEq<DiscontinuedIriOrLabel> for &str {
	fn eq(&self, other: &DiscontinuedIriOrLabel) -> bool {
		*self == DiscontinuedIri || *self == DISCONTINUED_LABEL
	}
}
