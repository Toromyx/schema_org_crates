/// <https://schema.org/partySize>
pub const PARTY_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/partySize";
/// <https://schema.org/partySize>
pub const PARTY_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partySize";
/// <https://schema.org/partySize>
pub const PARTY_SIZE_PROPERTY_LABEL: &str = "partySize";
pub struct PartySizePropertyIri;
impl PartialEq<&str> for PartySizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARTY_SIZE_PROPERTY_IRI_HTTP || *other == PARTY_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartySizePropertyIri> for &str {
	fn eq(&self, other: &PartySizePropertyIri) -> bool {
		*self == PARTY_SIZE_PROPERTY_IRI_HTTP || *self == PARTY_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct PartySizePropertyIriOrLabel;
impl PartialEq<&str> for PartySizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartySizePropertyIri || *other == PARTY_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<PartySizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartySizePropertyIriOrLabel) -> bool {
		*self == PartySizePropertyIri || *self == PARTY_SIZE_PROPERTY_LABEL
	}
}
