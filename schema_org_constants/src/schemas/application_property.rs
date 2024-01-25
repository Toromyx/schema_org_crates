/// <https://schema.org/application>
#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
pub const APPLICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/application";
/// <https://schema.org/application>
#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
pub const APPLICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/application";
/// <https://schema.org/application>
#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
pub const APPLICATION_PROPERTY_LABEL: &str = "application";
pub struct ApplicationPropertyIri;
impl PartialEq<&str> for ApplicationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_PROPERTY_IRI_HTTP || *other == APPLICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationPropertyIri> for &str {
	fn eq(&self, other: &ApplicationPropertyIri) -> bool {
		*self == APPLICATION_PROPERTY_IRI_HTTP || *self == APPLICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationPropertyIriOrLabel;
impl PartialEq<&str> for ApplicationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationPropertyIri || *other == APPLICATION_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationPropertyIriOrLabel) -> bool {
		*self == ApplicationPropertyIri || *self == APPLICATION_PROPERTY_LABEL
	}
}
