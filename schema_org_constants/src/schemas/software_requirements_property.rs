/// <https://schema.org/softwareRequirements>
pub const SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/softwareRequirements";
/// <https://schema.org/softwareRequirements>
pub const SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/softwareRequirements";
/// <https://schema.org/softwareRequirements>
pub const SOFTWARE_REQUIREMENTS_PROPERTY_LABEL: &str = "softwareRequirements";
pub struct SoftwareRequirementsPropertyIri;
impl PartialEq<&str> for SoftwareRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SoftwareRequirementsPropertyIri> for &str {
	fn eq(&self, other: &SoftwareRequirementsPropertyIri) -> bool {
		*self == SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == SOFTWARE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct SoftwareRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for SoftwareRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareRequirementsPropertyIri || *other == SOFTWARE_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<SoftwareRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareRequirementsPropertyIriOrLabel) -> bool {
		*self == SoftwareRequirementsPropertyIri || *self == SOFTWARE_REQUIREMENTS_PROPERTY_LABEL
	}
}
