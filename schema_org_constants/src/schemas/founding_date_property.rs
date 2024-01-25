/// <https://schema.org/foundingDate>
pub const FOUNDING_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/foundingDate";
/// <https://schema.org/foundingDate>
pub const FOUNDING_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/foundingDate";
/// <https://schema.org/foundingDate>
pub const FOUNDING_DATE_PROPERTY_LABEL: &str = "foundingDate";
pub struct FoundingDatePropertyIri;
impl PartialEq<&str> for FoundingDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOUNDING_DATE_PROPERTY_IRI_HTTP || *other == FOUNDING_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoundingDatePropertyIri> for &str {
	fn eq(&self, other: &FoundingDatePropertyIri) -> bool {
		*self == FOUNDING_DATE_PROPERTY_IRI_HTTP || *self == FOUNDING_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct FoundingDatePropertyIriOrLabel;
impl PartialEq<&str> for FoundingDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoundingDatePropertyIri || *other == FOUNDING_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<FoundingDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoundingDatePropertyIriOrLabel) -> bool {
		*self == FoundingDatePropertyIri || *self == FOUNDING_DATE_PROPERTY_LABEL
	}
}
