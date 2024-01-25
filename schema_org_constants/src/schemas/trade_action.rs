/// <https://schema.org/TradeAction>
pub const TRADE_ACTION_IRI_HTTP: &str = "http://schema.org/TradeAction";
/// <https://schema.org/TradeAction>
pub const TRADE_ACTION_IRI_HTTPS: &str = "https://schema.org/TradeAction";
/// <https://schema.org/TradeAction>
pub const TRADE_ACTION_LABEL: &str = "TradeAction";
pub struct TradeActionIri;
impl PartialEq<&str> for TradeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRADE_ACTION_IRI_HTTP || *other == TRADE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TradeActionIri> for &str {
	fn eq(&self, other: &TradeActionIri) -> bool {
		*self == TRADE_ACTION_IRI_HTTP || *self == TRADE_ACTION_IRI_HTTPS
	}
}
pub struct TradeActionIriOrLabel;
impl PartialEq<&str> for TradeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TradeActionIri || *other == TRADE_ACTION_LABEL
	}
}
impl PartialEq<TradeActionIriOrLabel> for &str {
	fn eq(&self, other: &TradeActionIriOrLabel) -> bool {
		*self == TradeActionIri || *self == TRADE_ACTION_LABEL
	}
}
