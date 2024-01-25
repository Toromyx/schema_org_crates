/// <https://schema.org/applicationSuite>
pub const APPLICATION_SUITE_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicationSuite";
/// <https://schema.org/applicationSuite>
pub const APPLICATION_SUITE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicationSuite";
/// <https://schema.org/applicationSuite>
pub const APPLICATION_SUITE_PROPERTY_LABEL: &str = "applicationSuite";
pub struct ApplicationSuitePropertyIri;
impl PartialEq<&str> for ApplicationSuitePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_SUITE_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_SUITE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationSuitePropertyIri> for &str {
	fn eq(&self, other: &ApplicationSuitePropertyIri) -> bool {
		*self == APPLICATION_SUITE_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_SUITE_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationSuitePropertyIriOrLabel;
impl PartialEq<&str> for ApplicationSuitePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationSuitePropertyIri || *other == APPLICATION_SUITE_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationSuitePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationSuitePropertyIriOrLabel) -> bool {
		*self == ApplicationSuitePropertyIri || *self == APPLICATION_SUITE_PROPERTY_LABEL
	}
}
