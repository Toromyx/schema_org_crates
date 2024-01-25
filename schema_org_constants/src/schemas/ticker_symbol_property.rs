/// <https://schema.org/tickerSymbol>
pub const TICKER_SYMBOL_PROPERTY_IRI_HTTP: &str = "http://schema.org/tickerSymbol";
/// <https://schema.org/tickerSymbol>
pub const TICKER_SYMBOL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tickerSymbol";
/// <https://schema.org/tickerSymbol>
pub const TICKER_SYMBOL_PROPERTY_LABEL: &str = "tickerSymbol";
pub struct TickerSymbolPropertyIri;
impl PartialEq<&str> for TickerSymbolPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TICKER_SYMBOL_PROPERTY_IRI_HTTP || *other == TICKER_SYMBOL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TickerSymbolPropertyIri> for &str {
	fn eq(&self, other: &TickerSymbolPropertyIri) -> bool {
		*self == TICKER_SYMBOL_PROPERTY_IRI_HTTP || *self == TICKER_SYMBOL_PROPERTY_IRI_HTTPS
	}
}
pub struct TickerSymbolPropertyIriOrLabel;
impl PartialEq<&str> for TickerSymbolPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TickerSymbolPropertyIri || *other == TICKER_SYMBOL_PROPERTY_LABEL
	}
}
impl PartialEq<TickerSymbolPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TickerSymbolPropertyIriOrLabel) -> bool {
		*self == TickerSymbolPropertyIri || *self == TICKER_SYMBOL_PROPERTY_LABEL
	}
}
