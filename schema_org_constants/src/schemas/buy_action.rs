/// <https://schema.org/BuyAction>
pub const BUY_ACTION_IRI_HTTP: &str = "http://schema.org/BuyAction";
/// <https://schema.org/BuyAction>
pub const BUY_ACTION_IRI_HTTPS: &str = "https://schema.org/BuyAction";
/// <https://schema.org/BuyAction>
pub const BUY_ACTION_LABEL: &str = "BuyAction";
pub struct BuyActionIri;
impl PartialEq<&str> for BuyActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUY_ACTION_IRI_HTTP || *other == BUY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<BuyActionIri> for &str {
	fn eq(&self, other: &BuyActionIri) -> bool {
		*self == BUY_ACTION_IRI_HTTP || *self == BUY_ACTION_IRI_HTTPS
	}
}
pub struct BuyActionIriOrLabel;
impl PartialEq<&str> for BuyActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BuyActionIri || *other == BUY_ACTION_LABEL
	}
}
impl PartialEq<BuyActionIriOrLabel> for &str {
	fn eq(&self, other: &BuyActionIriOrLabel) -> bool {
		*self == BuyActionIri || *self == BUY_ACTION_LABEL
	}
}
