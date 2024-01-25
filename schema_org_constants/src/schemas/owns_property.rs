/// <https://schema.org/owns>
pub const OWNS_PROPERTY_IRI_HTTP: &str = "http://schema.org/owns";
/// <https://schema.org/owns>
pub const OWNS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/owns";
/// <https://schema.org/owns>
pub const OWNS_PROPERTY_LABEL: &str = "owns";
pub struct OwnsPropertyIri;
impl PartialEq<&str> for OwnsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OWNS_PROPERTY_IRI_HTTP || *other == OWNS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OwnsPropertyIri> for &str {
	fn eq(&self, other: &OwnsPropertyIri) -> bool {
		*self == OWNS_PROPERTY_IRI_HTTP || *self == OWNS_PROPERTY_IRI_HTTPS
	}
}
pub struct OwnsPropertyIriOrLabel;
impl PartialEq<&str> for OwnsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OwnsPropertyIri || *other == OWNS_PROPERTY_LABEL
	}
}
impl PartialEq<OwnsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OwnsPropertyIriOrLabel) -> bool {
		*self == OwnsPropertyIri || *self == OWNS_PROPERTY_LABEL
	}
}
