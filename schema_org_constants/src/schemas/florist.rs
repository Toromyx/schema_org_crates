/// <https://schema.org/Florist>
pub const FLORIST_IRI_HTTP: &str = "http://schema.org/Florist";
/// <https://schema.org/Florist>
pub const FLORIST_IRI_HTTPS: &str = "https://schema.org/Florist";
/// <https://schema.org/Florist>
pub const FLORIST_LABEL: &str = "Florist";
pub struct FloristIri;
impl PartialEq<&str> for FloristIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLORIST_IRI_HTTP || *other == FLORIST_IRI_HTTPS
	}
}
impl PartialEq<FloristIri> for &str {
	fn eq(&self, other: &FloristIri) -> bool {
		*self == FLORIST_IRI_HTTP || *self == FLORIST_IRI_HTTPS
	}
}
pub struct FloristIriOrLabel;
impl PartialEq<&str> for FloristIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloristIri || *other == FLORIST_LABEL
	}
}
impl PartialEq<FloristIriOrLabel> for &str {
	fn eq(&self, other: &FloristIriOrLabel) -> bool {
		*self == FloristIri || *self == FLORIST_LABEL
	}
}
