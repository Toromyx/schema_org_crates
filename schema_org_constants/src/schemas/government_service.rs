/// <https://schema.org/GovernmentService>
pub const GOVERNMENT_SERVICE_IRI_HTTP: &str = "http://schema.org/GovernmentService";
/// <https://schema.org/GovernmentService>
pub const GOVERNMENT_SERVICE_IRI_HTTPS: &str = "https://schema.org/GovernmentService";
/// <https://schema.org/GovernmentService>
pub const GOVERNMENT_SERVICE_LABEL: &str = "GovernmentService";
pub struct GovernmentServiceIri;
impl PartialEq<&str> for GovernmentServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_SERVICE_IRI_HTTP || *other == GOVERNMENT_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<GovernmentServiceIri> for &str {
	fn eq(&self, other: &GovernmentServiceIri) -> bool {
		*self == GOVERNMENT_SERVICE_IRI_HTTP || *self == GOVERNMENT_SERVICE_IRI_HTTPS
	}
}
pub struct GovernmentServiceIriOrLabel;
impl PartialEq<&str> for GovernmentServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentServiceIri || *other == GOVERNMENT_SERVICE_LABEL
	}
}
impl PartialEq<GovernmentServiceIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentServiceIriOrLabel) -> bool {
		*self == GovernmentServiceIri || *self == GOVERNMENT_SERVICE_LABEL
	}
}
