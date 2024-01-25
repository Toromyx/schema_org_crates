/// <https://schema.org/offeredBy>
pub const OFFERED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/offeredBy";
/// <https://schema.org/offeredBy>
pub const OFFERED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/offeredBy";
/// <https://schema.org/offeredBy>
pub const OFFERED_BY_PROPERTY_LABEL: &str = "offeredBy";
pub struct OfferedByPropertyIri;
impl PartialEq<&str> for OfferedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFERED_BY_PROPERTY_IRI_HTTP || *other == OFFERED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OfferedByPropertyIri> for &str {
	fn eq(&self, other: &OfferedByPropertyIri) -> bool {
		*self == OFFERED_BY_PROPERTY_IRI_HTTP || *self == OFFERED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct OfferedByPropertyIriOrLabel;
impl PartialEq<&str> for OfferedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferedByPropertyIri || *other == OFFERED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<OfferedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OfferedByPropertyIriOrLabel) -> bool {
		*self == OfferedByPropertyIri || *self == OFFERED_BY_PROPERTY_LABEL
	}
}
