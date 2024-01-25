/// <https://schema.org/softwareAddOn>
pub const SOFTWARE_ADD_ON_PROPERTY_IRI_HTTP: &str = "http://schema.org/softwareAddOn";
/// <https://schema.org/softwareAddOn>
pub const SOFTWARE_ADD_ON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/softwareAddOn";
/// <https://schema.org/softwareAddOn>
pub const SOFTWARE_ADD_ON_PROPERTY_LABEL: &str = "softwareAddOn";
pub struct SoftwareAddOnPropertyIri;
impl PartialEq<&str> for SoftwareAddOnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_ADD_ON_PROPERTY_IRI_HTTP || *other == SOFTWARE_ADD_ON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SoftwareAddOnPropertyIri> for &str {
	fn eq(&self, other: &SoftwareAddOnPropertyIri) -> bool {
		*self == SOFTWARE_ADD_ON_PROPERTY_IRI_HTTP || *self == SOFTWARE_ADD_ON_PROPERTY_IRI_HTTPS
	}
}
pub struct SoftwareAddOnPropertyIriOrLabel;
impl PartialEq<&str> for SoftwareAddOnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareAddOnPropertyIri || *other == SOFTWARE_ADD_ON_PROPERTY_LABEL
	}
}
impl PartialEq<SoftwareAddOnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareAddOnPropertyIriOrLabel) -> bool {
		*self == SoftwareAddOnPropertyIri || *self == SOFTWARE_ADD_ON_PROPERTY_LABEL
	}
}
