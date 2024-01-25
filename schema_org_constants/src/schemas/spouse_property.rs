/// <https://schema.org/spouse>
pub const SPOUSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/spouse";
/// <https://schema.org/spouse>
pub const SPOUSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/spouse";
/// <https://schema.org/spouse>
pub const SPOUSE_PROPERTY_LABEL: &str = "spouse";
pub struct SpousePropertyIri;
impl PartialEq<&str> for SpousePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPOUSE_PROPERTY_IRI_HTTP || *other == SPOUSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpousePropertyIri> for &str {
	fn eq(&self, other: &SpousePropertyIri) -> bool {
		*self == SPOUSE_PROPERTY_IRI_HTTP || *self == SPOUSE_PROPERTY_IRI_HTTPS
	}
}
pub struct SpousePropertyIriOrLabel;
impl PartialEq<&str> for SpousePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpousePropertyIri || *other == SPOUSE_PROPERTY_LABEL
	}
}
impl PartialEq<SpousePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpousePropertyIriOrLabel) -> bool {
		*self == SpousePropertyIri || *self == SPOUSE_PROPERTY_LABEL
	}
}
