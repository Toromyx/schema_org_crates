/// <https://schema.org/usesDevice>
pub const USES_DEVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/usesDevice";
/// <https://schema.org/usesDevice>
pub const USES_DEVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/usesDevice";
/// <https://schema.org/usesDevice>
pub const USES_DEVICE_PROPERTY_LABEL: &str = "usesDevice";
pub struct UsesDevicePropertyIri;
impl PartialEq<&str> for UsesDevicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USES_DEVICE_PROPERTY_IRI_HTTP || *other == USES_DEVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UsesDevicePropertyIri> for &str {
	fn eq(&self, other: &UsesDevicePropertyIri) -> bool {
		*self == USES_DEVICE_PROPERTY_IRI_HTTP || *self == USES_DEVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct UsesDevicePropertyIriOrLabel;
impl PartialEq<&str> for UsesDevicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsesDevicePropertyIri || *other == USES_DEVICE_PROPERTY_LABEL
	}
}
impl PartialEq<UsesDevicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UsesDevicePropertyIriOrLabel) -> bool {
		*self == UsesDevicePropertyIri || *self == USES_DEVICE_PROPERTY_LABEL
	}
}
