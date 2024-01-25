/// <https://schema.org/branchOf>
#[deprecated = "This schema is superseded by <https://schema.org/parentOrganization>."]
pub const BRANCH_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/branchOf";
/// <https://schema.org/branchOf>
#[deprecated = "This schema is superseded by <https://schema.org/parentOrganization>."]
pub const BRANCH_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/branchOf";
/// <https://schema.org/branchOf>
#[deprecated = "This schema is superseded by <https://schema.org/parentOrganization>."]
pub const BRANCH_OF_PROPERTY_LABEL: &str = "branchOf";
pub struct BranchOfPropertyIri;
impl PartialEq<&str> for BranchOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRANCH_OF_PROPERTY_IRI_HTTP || *other == BRANCH_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BranchOfPropertyIri> for &str {
	fn eq(&self, other: &BranchOfPropertyIri) -> bool {
		*self == BRANCH_OF_PROPERTY_IRI_HTTP || *self == BRANCH_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct BranchOfPropertyIriOrLabel;
impl PartialEq<&str> for BranchOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BranchOfPropertyIri || *other == BRANCH_OF_PROPERTY_LABEL
	}
}
impl PartialEq<BranchOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BranchOfPropertyIriOrLabel) -> bool {
		*self == BranchOfPropertyIri || *self == BRANCH_OF_PROPERTY_LABEL
	}
}
