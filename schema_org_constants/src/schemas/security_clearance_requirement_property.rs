/// <https://schema.org/securityClearanceRequirement>
pub const SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/securityClearanceRequirement";
/// <https://schema.org/securityClearanceRequirement>
pub const SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/securityClearanceRequirement";
/// <https://schema.org/securityClearanceRequirement>
pub const SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_LABEL: &str = "securityClearanceRequirement";
pub struct SecurityClearanceRequirementPropertyIri;
impl PartialEq<&str> for SecurityClearanceRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SecurityClearanceRequirementPropertyIri> for &str {
	fn eq(&self, other: &SecurityClearanceRequirementPropertyIri) -> bool {
		*self == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SecurityClearanceRequirementPropertyIriOrLabel;
impl PartialEq<&str> for SecurityClearanceRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SecurityClearanceRequirementPropertyIri
			|| *other == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<SecurityClearanceRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SecurityClearanceRequirementPropertyIriOrLabel) -> bool {
		*self == SecurityClearanceRequirementPropertyIri
			|| *self == SECURITY_CLEARANCE_REQUIREMENT_PROPERTY_LABEL
	}
}
