/// <https://schema.org/TypeAndQuantityNode>
pub const TYPE_AND_QUANTITY_NODE_IRI_HTTP: &str = "http://schema.org/TypeAndQuantityNode";
/// <https://schema.org/TypeAndQuantityNode>
pub const TYPE_AND_QUANTITY_NODE_IRI_HTTPS: &str = "https://schema.org/TypeAndQuantityNode";
/// <https://schema.org/TypeAndQuantityNode>
pub const TYPE_AND_QUANTITY_NODE_LABEL: &str = "TypeAndQuantityNode";
pub struct TypeAndQuantityNodeIri;
impl PartialEq<&str> for TypeAndQuantityNodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPE_AND_QUANTITY_NODE_IRI_HTTP || *other == TYPE_AND_QUANTITY_NODE_IRI_HTTPS
	}
}
impl PartialEq<TypeAndQuantityNodeIri> for &str {
	fn eq(&self, other: &TypeAndQuantityNodeIri) -> bool {
		*self == TYPE_AND_QUANTITY_NODE_IRI_HTTP || *self == TYPE_AND_QUANTITY_NODE_IRI_HTTPS
	}
}
pub struct TypeAndQuantityNodeIriOrLabel;
impl PartialEq<&str> for TypeAndQuantityNodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypeAndQuantityNodeIri || *other == TYPE_AND_QUANTITY_NODE_LABEL
	}
}
impl PartialEq<TypeAndQuantityNodeIriOrLabel> for &str {
	fn eq(&self, other: &TypeAndQuantityNodeIriOrLabel) -> bool {
		*self == TypeAndQuantityNodeIri || *self == TYPE_AND_QUANTITY_NODE_LABEL
	}
}
