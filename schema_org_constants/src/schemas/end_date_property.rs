/// <https://schema.org/endDate>
pub const END_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/endDate";
/// <https://schema.org/endDate>
pub const END_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/endDate";
/// <https://schema.org/endDate>
pub const END_DATE_PROPERTY_LABEL: &str = "endDate";
pub struct EndDatePropertyIri;
impl PartialEq<&str> for EndDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == END_DATE_PROPERTY_IRI_HTTP || *other == END_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EndDatePropertyIri> for &str {
	fn eq(&self, other: &EndDatePropertyIri) -> bool {
		*self == END_DATE_PROPERTY_IRI_HTTP || *self == END_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct EndDatePropertyIriOrLabel;
impl PartialEq<&str> for EndDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndDatePropertyIri || *other == END_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<EndDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EndDatePropertyIriOrLabel) -> bool {
		*self == EndDatePropertyIri || *self == END_DATE_PROPERTY_LABEL
	}
}
