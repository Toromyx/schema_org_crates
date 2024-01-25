/// <https://schema.org/PreOrder>
pub const PRE_ORDER_IRI_HTTP: &str = "http://schema.org/PreOrder";
/// <https://schema.org/PreOrder>
pub const PRE_ORDER_IRI_HTTPS: &str = "https://schema.org/PreOrder";
/// <https://schema.org/PreOrder>
pub const PRE_ORDER_LABEL: &str = "PreOrder";
pub struct PreOrderIri;
impl PartialEq<&str> for PreOrderIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRE_ORDER_IRI_HTTP || *other == PRE_ORDER_IRI_HTTPS
	}
}
impl PartialEq<PreOrderIri> for &str {
	fn eq(&self, other: &PreOrderIri) -> bool {
		*self == PRE_ORDER_IRI_HTTP || *self == PRE_ORDER_IRI_HTTPS
	}
}
pub struct PreOrderIriOrLabel;
impl PartialEq<&str> for PreOrderIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreOrderIri || *other == PRE_ORDER_LABEL
	}
}
impl PartialEq<PreOrderIriOrLabel> for &str {
	fn eq(&self, other: &PreOrderIriOrLabel) -> bool {
		*self == PreOrderIri || *self == PRE_ORDER_LABEL
	}
}
