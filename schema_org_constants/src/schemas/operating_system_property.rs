/// <https://schema.org/operatingSystem>
pub const OPERATING_SYSTEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/operatingSystem";
/// <https://schema.org/operatingSystem>
pub const OPERATING_SYSTEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/operatingSystem";
/// <https://schema.org/operatingSystem>
pub const OPERATING_SYSTEM_PROPERTY_LABEL: &str = "operatingSystem";
pub struct OperatingSystemPropertyIri;
impl PartialEq<&str> for OperatingSystemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPERATING_SYSTEM_PROPERTY_IRI_HTTP
			|| *other == OPERATING_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OperatingSystemPropertyIri> for &str {
	fn eq(&self, other: &OperatingSystemPropertyIri) -> bool {
		*self == OPERATING_SYSTEM_PROPERTY_IRI_HTTP || *self == OPERATING_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
pub struct OperatingSystemPropertyIriOrLabel;
impl PartialEq<&str> for OperatingSystemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OperatingSystemPropertyIri || *other == OPERATING_SYSTEM_PROPERTY_LABEL
	}
}
impl PartialEq<OperatingSystemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OperatingSystemPropertyIriOrLabel) -> bool {
		*self == OperatingSystemPropertyIri || *self == OPERATING_SYSTEM_PROPERTY_LABEL
	}
}
