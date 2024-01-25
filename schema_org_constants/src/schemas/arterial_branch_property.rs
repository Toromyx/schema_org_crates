/// <https://schema.org/arterialBranch>
pub const ARTERIAL_BRANCH_PROPERTY_IRI_HTTP: &str = "http://schema.org/arterialBranch";
/// <https://schema.org/arterialBranch>
pub const ARTERIAL_BRANCH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arterialBranch";
/// <https://schema.org/arterialBranch>
pub const ARTERIAL_BRANCH_PROPERTY_LABEL: &str = "arterialBranch";
pub struct ArterialBranchPropertyIri;
impl PartialEq<&str> for ArterialBranchPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTERIAL_BRANCH_PROPERTY_IRI_HTTP || *other == ARTERIAL_BRANCH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArterialBranchPropertyIri> for &str {
	fn eq(&self, other: &ArterialBranchPropertyIri) -> bool {
		*self == ARTERIAL_BRANCH_PROPERTY_IRI_HTTP || *self == ARTERIAL_BRANCH_PROPERTY_IRI_HTTPS
	}
}
pub struct ArterialBranchPropertyIriOrLabel;
impl PartialEq<&str> for ArterialBranchPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArterialBranchPropertyIri || *other == ARTERIAL_BRANCH_PROPERTY_LABEL
	}
}
impl PartialEq<ArterialBranchPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArterialBranchPropertyIriOrLabel) -> bool {
		*self == ArterialBranchPropertyIri || *self == ARTERIAL_BRANCH_PROPERTY_LABEL
	}
}
