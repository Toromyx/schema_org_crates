/// <https://schema.org/branchCode>
pub const BRANCH_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/branchCode";
/// <https://schema.org/branchCode>
pub const BRANCH_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/branchCode";
/// <https://schema.org/branchCode>
pub const BRANCH_CODE_PROPERTY_LABEL: &str = "branchCode";
pub struct BranchCodePropertyIri;
impl PartialEq<&str> for BranchCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRANCH_CODE_PROPERTY_IRI_HTTP || *other == BRANCH_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BranchCodePropertyIri> for &str {
	fn eq(&self, other: &BranchCodePropertyIri) -> bool {
		*self == BRANCH_CODE_PROPERTY_IRI_HTTP || *self == BRANCH_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct BranchCodePropertyIriOrLabel;
impl PartialEq<&str> for BranchCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BranchCodePropertyIri || *other == BRANCH_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<BranchCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BranchCodePropertyIriOrLabel) -> bool {
		*self == BranchCodePropertyIri || *self == BRANCH_CODE_PROPERTY_LABEL
	}
}
