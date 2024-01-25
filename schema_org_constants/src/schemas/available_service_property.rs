/// <https://schema.org/availableService>
pub const AVAILABLE_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableService";
/// <https://schema.org/availableService>
pub const AVAILABLE_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableService";
/// <https://schema.org/availableService>
pub const AVAILABLE_SERVICE_PROPERTY_LABEL: &str = "availableService";
pub struct AvailableServicePropertyIri;
impl PartialEq<&str> for AvailableServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_SERVICE_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableServicePropertyIri> for &str {
	fn eq(&self, other: &AvailableServicePropertyIri) -> bool {
		*self == AVAILABLE_SERVICE_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableServicePropertyIriOrLabel;
impl PartialEq<&str> for AvailableServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableServicePropertyIri || *other == AVAILABLE_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableServicePropertyIriOrLabel) -> bool {
		*self == AvailableServicePropertyIri || *self == AVAILABLE_SERVICE_PROPERTY_LABEL
	}
}
