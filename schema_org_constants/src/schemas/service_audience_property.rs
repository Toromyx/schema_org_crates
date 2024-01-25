/// <https://schema.org/serviceAudience>
#[deprecated = "This schema is superseded by <https://schema.org/audience>."]
pub const SERVICE_AUDIENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceAudience";
/// <https://schema.org/serviceAudience>
#[deprecated = "This schema is superseded by <https://schema.org/audience>."]
pub const SERVICE_AUDIENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceAudience";
/// <https://schema.org/serviceAudience>
#[deprecated = "This schema is superseded by <https://schema.org/audience>."]
pub const SERVICE_AUDIENCE_PROPERTY_LABEL: &str = "serviceAudience";
pub struct ServiceAudiencePropertyIri;
impl PartialEq<&str> for ServiceAudiencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_AUDIENCE_PROPERTY_IRI_HTTP
			|| *other == SERVICE_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceAudiencePropertyIri> for &str {
	fn eq(&self, other: &ServiceAudiencePropertyIri) -> bool {
		*self == SERVICE_AUDIENCE_PROPERTY_IRI_HTTP || *self == SERVICE_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceAudiencePropertyIriOrLabel;
impl PartialEq<&str> for ServiceAudiencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceAudiencePropertyIri || *other == SERVICE_AUDIENCE_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceAudiencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceAudiencePropertyIriOrLabel) -> bool {
		*self == ServiceAudiencePropertyIri || *self == SERVICE_AUDIENCE_PROPERTY_LABEL
	}
}
