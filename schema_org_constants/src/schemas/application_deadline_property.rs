/// <https://schema.org/applicationDeadline>
pub const APPLICATION_DEADLINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicationDeadline";
/// <https://schema.org/applicationDeadline>
pub const APPLICATION_DEADLINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicationDeadline";
/// <https://schema.org/applicationDeadline>
pub const APPLICATION_DEADLINE_PROPERTY_LABEL: &str = "applicationDeadline";
pub struct ApplicationDeadlinePropertyIri;
impl PartialEq<&str> for ApplicationDeadlinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_DEADLINE_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_DEADLINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationDeadlinePropertyIri> for &str {
	fn eq(&self, other: &ApplicationDeadlinePropertyIri) -> bool {
		*self == APPLICATION_DEADLINE_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_DEADLINE_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationDeadlinePropertyIriOrLabel;
impl PartialEq<&str> for ApplicationDeadlinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationDeadlinePropertyIri || *other == APPLICATION_DEADLINE_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationDeadlinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationDeadlinePropertyIriOrLabel) -> bool {
		*self == ApplicationDeadlinePropertyIri || *self == APPLICATION_DEADLINE_PROPERTY_LABEL
	}
}
