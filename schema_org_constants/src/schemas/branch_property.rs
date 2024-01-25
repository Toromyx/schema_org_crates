/// <https://schema.org/branch>
#[deprecated = "This schema is superseded by <https://schema.org/arterialBranch>."]
pub const BRANCH_PROPERTY_IRI_HTTP: &str = "http://schema.org/branch";
/// <https://schema.org/branch>
#[deprecated = "This schema is superseded by <https://schema.org/arterialBranch>."]
pub const BRANCH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/branch";
/// <https://schema.org/branch>
#[deprecated = "This schema is superseded by <https://schema.org/arterialBranch>."]
pub const BRANCH_PROPERTY_LABEL: &str = "branch";
pub struct BranchPropertyIri;
impl PartialEq<&str> for BranchPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRANCH_PROPERTY_IRI_HTTP || *other == BRANCH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BranchPropertyIri> for &str {
	fn eq(&self, other: &BranchPropertyIri) -> bool {
		*self == BRANCH_PROPERTY_IRI_HTTP || *self == BRANCH_PROPERTY_IRI_HTTPS
	}
}
pub struct BranchPropertyIriOrLabel;
impl PartialEq<&str> for BranchPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BranchPropertyIri || *other == BRANCH_PROPERTY_LABEL
	}
}
impl PartialEq<BranchPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BranchPropertyIriOrLabel) -> bool {
		*self == BranchPropertyIri || *self == BRANCH_PROPERTY_LABEL
	}
}
