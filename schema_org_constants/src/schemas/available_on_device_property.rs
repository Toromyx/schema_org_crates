/// <https://schema.org/availableOnDevice>
pub const AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableOnDevice";
/// <https://schema.org/availableOnDevice>
pub const AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableOnDevice";
/// <https://schema.org/availableOnDevice>
pub const AVAILABLE_ON_DEVICE_PROPERTY_LABEL: &str = "availableOnDevice";
pub struct AvailableOnDevicePropertyIri;
impl PartialEq<&str> for AvailableOnDevicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableOnDevicePropertyIri> for &str {
	fn eq(&self, other: &AvailableOnDevicePropertyIri) -> bool {
		*self == AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_ON_DEVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableOnDevicePropertyIriOrLabel;
impl PartialEq<&str> for AvailableOnDevicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableOnDevicePropertyIri || *other == AVAILABLE_ON_DEVICE_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableOnDevicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableOnDevicePropertyIriOrLabel) -> bool {
		*self == AvailableOnDevicePropertyIri || *self == AVAILABLE_ON_DEVICE_PROPERTY_LABEL
	}
}
