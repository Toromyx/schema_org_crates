/// <https://schema.org/startDate>
pub const START_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/startDate";
/// <https://schema.org/startDate>
pub const START_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/startDate";
/// <https://schema.org/startDate>
pub const START_DATE_PROPERTY_LABEL: &str = "startDate";
pub struct StartDatePropertyIri;
impl PartialEq<&str> for StartDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == START_DATE_PROPERTY_IRI_HTTP || *other == START_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StartDatePropertyIri> for &str {
	fn eq(&self, other: &StartDatePropertyIri) -> bool {
		*self == START_DATE_PROPERTY_IRI_HTTP || *self == START_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct StartDatePropertyIriOrLabel;
impl PartialEq<&str> for StartDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StartDatePropertyIri || *other == START_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<StartDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StartDatePropertyIriOrLabel) -> bool {
		*self == StartDatePropertyIri || *self == START_DATE_PROPERTY_LABEL
	}
}
