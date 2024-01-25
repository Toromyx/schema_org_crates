/// <https://schema.org/applicationContact>
pub const APPLICATION_CONTACT_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicationContact";
/// <https://schema.org/applicationContact>
pub const APPLICATION_CONTACT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicationContact";
/// <https://schema.org/applicationContact>
pub const APPLICATION_CONTACT_PROPERTY_LABEL: &str = "applicationContact";
pub struct ApplicationContactPropertyIri;
impl PartialEq<&str> for ApplicationContactPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_CONTACT_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_CONTACT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationContactPropertyIri> for &str {
	fn eq(&self, other: &ApplicationContactPropertyIri) -> bool {
		*self == APPLICATION_CONTACT_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_CONTACT_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationContactPropertyIriOrLabel;
impl PartialEq<&str> for ApplicationContactPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationContactPropertyIri || *other == APPLICATION_CONTACT_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationContactPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationContactPropertyIriOrLabel) -> bool {
		*self == ApplicationContactPropertyIri || *self == APPLICATION_CONTACT_PROPERTY_LABEL
	}
}
