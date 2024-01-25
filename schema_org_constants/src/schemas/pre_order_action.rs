/// <https://schema.org/PreOrderAction>
pub const PRE_ORDER_ACTION_IRI_HTTP: &str = "http://schema.org/PreOrderAction";
/// <https://schema.org/PreOrderAction>
pub const PRE_ORDER_ACTION_IRI_HTTPS: &str = "https://schema.org/PreOrderAction";
/// <https://schema.org/PreOrderAction>
pub const PRE_ORDER_ACTION_LABEL: &str = "PreOrderAction";
pub struct PreOrderActionIri;
impl PartialEq<&str> for PreOrderActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRE_ORDER_ACTION_IRI_HTTP || *other == PRE_ORDER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PreOrderActionIri> for &str {
	fn eq(&self, other: &PreOrderActionIri) -> bool {
		*self == PRE_ORDER_ACTION_IRI_HTTP || *self == PRE_ORDER_ACTION_IRI_HTTPS
	}
}
pub struct PreOrderActionIriOrLabel;
impl PartialEq<&str> for PreOrderActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreOrderActionIri || *other == PRE_ORDER_ACTION_LABEL
	}
}
impl PartialEq<PreOrderActionIriOrLabel> for &str {
	fn eq(&self, other: &PreOrderActionIriOrLabel) -> bool {
		*self == PreOrderActionIri || *self == PRE_ORDER_ACTION_LABEL
	}
}
