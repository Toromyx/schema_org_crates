/// <https://schema.org/physicalRequirement>
pub const PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/physicalRequirement";
/// <https://schema.org/physicalRequirement>
pub const PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/physicalRequirement";
/// <https://schema.org/physicalRequirement>
pub const PHYSICAL_REQUIREMENT_PROPERTY_LABEL: &str = "physicalRequirement";
pub struct PhysicalRequirementPropertyIri;
impl PartialEq<&str> for PhysicalRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PhysicalRequirementPropertyIri> for &str {
	fn eq(&self, other: &PhysicalRequirementPropertyIri) -> bool {
		*self == PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == PHYSICAL_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct PhysicalRequirementPropertyIriOrLabel;
impl PartialEq<&str> for PhysicalRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicalRequirementPropertyIri || *other == PHYSICAL_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<PhysicalRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PhysicalRequirementPropertyIriOrLabel) -> bool {
		*self == PhysicalRequirementPropertyIri || *self == PHYSICAL_REQUIREMENT_PROPERTY_LABEL
	}
}
