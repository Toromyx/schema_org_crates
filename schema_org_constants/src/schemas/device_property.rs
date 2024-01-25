/// <https://schema.org/device>
#[deprecated = "This schema is superseded by <https://schema.org/availableOnDevice>."]
pub const DEVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/device";
/// <https://schema.org/device>
#[deprecated = "This schema is superseded by <https://schema.org/availableOnDevice>."]
pub const DEVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/device";
/// <https://schema.org/device>
#[deprecated = "This schema is superseded by <https://schema.org/availableOnDevice>."]
pub const DEVICE_PROPERTY_LABEL: &str = "device";
pub struct DevicePropertyIri;
impl PartialEq<&str> for DevicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEVICE_PROPERTY_IRI_HTTP || *other == DEVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DevicePropertyIri> for &str {
	fn eq(&self, other: &DevicePropertyIri) -> bool {
		*self == DEVICE_PROPERTY_IRI_HTTP || *self == DEVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct DevicePropertyIriOrLabel;
impl PartialEq<&str> for DevicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DevicePropertyIri || *other == DEVICE_PROPERTY_LABEL
	}
}
impl PartialEq<DevicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DevicePropertyIriOrLabel) -> bool {
		*self == DevicePropertyIri || *self == DEVICE_PROPERTY_LABEL
	}
}
