/// <https://schema.org/applicationStartDate>
pub const APPLICATION_START_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicationStartDate";
/// <https://schema.org/applicationStartDate>
pub const APPLICATION_START_DATE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/applicationStartDate";
/// <https://schema.org/applicationStartDate>
pub const APPLICATION_START_DATE_PROPERTY_LABEL: &str = "applicationStartDate";
pub struct ApplicationStartDatePropertyIri;
impl PartialEq<&str> for ApplicationStartDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_START_DATE_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_START_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationStartDatePropertyIri> for &str {
	fn eq(&self, other: &ApplicationStartDatePropertyIri) -> bool {
		*self == APPLICATION_START_DATE_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_START_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationStartDatePropertyIriOrLabel;
impl PartialEq<&str> for ApplicationStartDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationStartDatePropertyIri || *other == APPLICATION_START_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationStartDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationStartDatePropertyIriOrLabel) -> bool {
		*self == ApplicationStartDatePropertyIri || *self == APPLICATION_START_DATE_PROPERTY_LABEL
	}
}
