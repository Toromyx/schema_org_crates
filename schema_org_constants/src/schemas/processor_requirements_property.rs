/// <https://schema.org/processorRequirements>
pub const PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/processorRequirements";
/// <https://schema.org/processorRequirements>
pub const PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/processorRequirements";
/// <https://schema.org/processorRequirements>
pub const PROCESSOR_REQUIREMENTS_PROPERTY_LABEL: &str = "processorRequirements";
pub struct ProcessorRequirementsPropertyIri;
impl PartialEq<&str> for ProcessorRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProcessorRequirementsPropertyIri> for &str {
	fn eq(&self, other: &ProcessorRequirementsPropertyIri) -> bool {
		*self == PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == PROCESSOR_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct ProcessorRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for ProcessorRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProcessorRequirementsPropertyIri
			|| *other == PROCESSOR_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<ProcessorRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProcessorRequirementsPropertyIriOrLabel) -> bool {
		*self == ProcessorRequirementsPropertyIri || *self == PROCESSOR_REQUIREMENTS_PROPERTY_LABEL
	}
}
