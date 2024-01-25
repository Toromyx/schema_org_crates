/// <https://schema.org/SellAction>
pub const SELL_ACTION_IRI_HTTP: &str = "http://schema.org/SellAction";
/// <https://schema.org/SellAction>
pub const SELL_ACTION_IRI_HTTPS: &str = "https://schema.org/SellAction";
/// <https://schema.org/SellAction>
pub const SELL_ACTION_LABEL: &str = "SellAction";
pub struct SellActionIri;
impl PartialEq<&str> for SellActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SELL_ACTION_IRI_HTTP || *other == SELL_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SellActionIri> for &str {
	fn eq(&self, other: &SellActionIri) -> bool {
		*self == SELL_ACTION_IRI_HTTP || *self == SELL_ACTION_IRI_HTTPS
	}
}
pub struct SellActionIriOrLabel;
impl PartialEq<&str> for SellActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SellActionIri || *other == SELL_ACTION_LABEL
	}
}
impl PartialEq<SellActionIriOrLabel> for &str {
	fn eq(&self, other: &SellActionIriOrLabel) -> bool {
		*self == SellActionIri || *self == SELL_ACTION_LABEL
	}
}
