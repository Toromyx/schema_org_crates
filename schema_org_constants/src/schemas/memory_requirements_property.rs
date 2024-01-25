/// <https://schema.org/memoryRequirements>
pub const MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/memoryRequirements";
/// <https://schema.org/memoryRequirements>
pub const MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/memoryRequirements";
/// <https://schema.org/memoryRequirements>
pub const MEMORY_REQUIREMENTS_PROPERTY_LABEL: &str = "memoryRequirements";
pub struct MemoryRequirementsPropertyIri;
impl PartialEq<&str> for MemoryRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MemoryRequirementsPropertyIri> for &str {
	fn eq(&self, other: &MemoryRequirementsPropertyIri) -> bool {
		*self == MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == MEMORY_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct MemoryRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for MemoryRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MemoryRequirementsPropertyIri || *other == MEMORY_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<MemoryRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MemoryRequirementsPropertyIriOrLabel) -> bool {
		*self == MemoryRequirementsPropertyIri || *self == MEMORY_REQUIREMENTS_PROPERTY_LABEL
	}
}
