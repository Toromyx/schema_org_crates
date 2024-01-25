/// <https://schema.org/ConstraintNode>
pub const CONSTRAINT_NODE_IRI_HTTP: &str = "http://schema.org/ConstraintNode";
/// <https://schema.org/ConstraintNode>
pub const CONSTRAINT_NODE_IRI_HTTPS: &str = "https://schema.org/ConstraintNode";
/// <https://schema.org/ConstraintNode>
pub const CONSTRAINT_NODE_LABEL: &str = "ConstraintNode";
pub struct ConstraintNodeIri;
impl PartialEq<&str> for ConstraintNodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONSTRAINT_NODE_IRI_HTTP || *other == CONSTRAINT_NODE_IRI_HTTPS
	}
}
impl PartialEq<ConstraintNodeIri> for &str {
	fn eq(&self, other: &ConstraintNodeIri) -> bool {
		*self == CONSTRAINT_NODE_IRI_HTTP || *self == CONSTRAINT_NODE_IRI_HTTPS
	}
}
pub struct ConstraintNodeIriOrLabel;
impl PartialEq<&str> for ConstraintNodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConstraintNodeIri || *other == CONSTRAINT_NODE_LABEL
	}
}
impl PartialEq<ConstraintNodeIriOrLabel> for &str {
	fn eq(&self, other: &ConstraintNodeIriOrLabel) -> bool {
		*self == ConstraintNodeIri || *self == CONSTRAINT_NODE_LABEL
	}
}
