/// <https://schema.org/seeks>
pub const SEEKS_PROPERTY_IRI_HTTP: &str = "http://schema.org/seeks";
/// <https://schema.org/seeks>
pub const SEEKS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seeks";
/// <https://schema.org/seeks>
pub const SEEKS_PROPERTY_LABEL: &str = "seeks";
pub struct SeeksPropertyIri;
impl PartialEq<&str> for SeeksPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEEKS_PROPERTY_IRI_HTTP || *other == SEEKS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeeksPropertyIri> for &str {
	fn eq(&self, other: &SeeksPropertyIri) -> bool {
		*self == SEEKS_PROPERTY_IRI_HTTP || *self == SEEKS_PROPERTY_IRI_HTTPS
	}
}
pub struct SeeksPropertyIriOrLabel;
impl PartialEq<&str> for SeeksPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeeksPropertyIri || *other == SEEKS_PROPERTY_LABEL
	}
}
impl PartialEq<SeeksPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeeksPropertyIriOrLabel) -> bool {
		*self == SeeksPropertyIri || *self == SEEKS_PROPERTY_LABEL
	}
}
