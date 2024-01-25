/// <https://schema.org/softwareHelp>
pub const SOFTWARE_HELP_PROPERTY_IRI_HTTP: &str = "http://schema.org/softwareHelp";
/// <https://schema.org/softwareHelp>
pub const SOFTWARE_HELP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/softwareHelp";
/// <https://schema.org/softwareHelp>
pub const SOFTWARE_HELP_PROPERTY_LABEL: &str = "softwareHelp";
pub struct SoftwareHelpPropertyIri;
impl PartialEq<&str> for SoftwareHelpPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_HELP_PROPERTY_IRI_HTTP || *other == SOFTWARE_HELP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SoftwareHelpPropertyIri> for &str {
	fn eq(&self, other: &SoftwareHelpPropertyIri) -> bool {
		*self == SOFTWARE_HELP_PROPERTY_IRI_HTTP || *self == SOFTWARE_HELP_PROPERTY_IRI_HTTPS
	}
}
pub struct SoftwareHelpPropertyIriOrLabel;
impl PartialEq<&str> for SoftwareHelpPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareHelpPropertyIri || *other == SOFTWARE_HELP_PROPERTY_LABEL
	}
}
impl PartialEq<SoftwareHelpPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareHelpPropertyIriOrLabel) -> bool {
		*self == SoftwareHelpPropertyIri || *self == SOFTWARE_HELP_PROPERTY_LABEL
	}
}
