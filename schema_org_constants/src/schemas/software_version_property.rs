/// <https://schema.org/softwareVersion>
pub const SOFTWARE_VERSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/softwareVersion";
/// <https://schema.org/softwareVersion>
pub const SOFTWARE_VERSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/softwareVersion";
/// <https://schema.org/softwareVersion>
pub const SOFTWARE_VERSION_PROPERTY_LABEL: &str = "softwareVersion";
pub struct SoftwareVersionPropertyIri;
impl PartialEq<&str> for SoftwareVersionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_VERSION_PROPERTY_IRI_HTTP
			|| *other == SOFTWARE_VERSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SoftwareVersionPropertyIri> for &str {
	fn eq(&self, other: &SoftwareVersionPropertyIri) -> bool {
		*self == SOFTWARE_VERSION_PROPERTY_IRI_HTTP || *self == SOFTWARE_VERSION_PROPERTY_IRI_HTTPS
	}
}
pub struct SoftwareVersionPropertyIriOrLabel;
impl PartialEq<&str> for SoftwareVersionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareVersionPropertyIri || *other == SOFTWARE_VERSION_PROPERTY_LABEL
	}
}
impl PartialEq<SoftwareVersionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareVersionPropertyIriOrLabel) -> bool {
		*self == SoftwareVersionPropertyIri || *self == SOFTWARE_VERSION_PROPERTY_LABEL
	}
}
