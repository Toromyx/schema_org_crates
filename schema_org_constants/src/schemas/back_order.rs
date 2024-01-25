/// <https://schema.org/BackOrder>
pub const BACK_ORDER_IRI_HTTP: &str = "http://schema.org/BackOrder";
/// <https://schema.org/BackOrder>
pub const BACK_ORDER_IRI_HTTPS: &str = "https://schema.org/BackOrder";
/// <https://schema.org/BackOrder>
pub const BACK_ORDER_LABEL: &str = "BackOrder";
pub struct BackOrderIri;
impl PartialEq<&str> for BackOrderIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BACK_ORDER_IRI_HTTP || *other == BACK_ORDER_IRI_HTTPS
	}
}
impl PartialEq<BackOrderIri> for &str {
	fn eq(&self, other: &BackOrderIri) -> bool {
		*self == BACK_ORDER_IRI_HTTP || *self == BACK_ORDER_IRI_HTTPS
	}
}
pub struct BackOrderIriOrLabel;
impl PartialEq<&str> for BackOrderIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BackOrderIri || *other == BACK_ORDER_LABEL
	}
}
impl PartialEq<BackOrderIriOrLabel> for &str {
	fn eq(&self, other: &BackOrderIriOrLabel) -> bool {
		*self == BackOrderIri || *self == BACK_ORDER_LABEL
	}
}
