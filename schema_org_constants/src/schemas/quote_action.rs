/// <https://schema.org/QuoteAction>
pub const QUOTE_ACTION_IRI_HTTP: &str = "http://schema.org/QuoteAction";
/// <https://schema.org/QuoteAction>
pub const QUOTE_ACTION_IRI_HTTPS: &str = "https://schema.org/QuoteAction";
/// <https://schema.org/QuoteAction>
pub const QUOTE_ACTION_LABEL: &str = "QuoteAction";
pub struct QuoteActionIri;
impl PartialEq<&str> for QuoteActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUOTE_ACTION_IRI_HTTP || *other == QUOTE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<QuoteActionIri> for &str {
	fn eq(&self, other: &QuoteActionIri) -> bool {
		*self == QUOTE_ACTION_IRI_HTTP || *self == QUOTE_ACTION_IRI_HTTPS
	}
}
pub struct QuoteActionIriOrLabel;
impl PartialEq<&str> for QuoteActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuoteActionIri || *other == QUOTE_ACTION_LABEL
	}
}
impl PartialEq<QuoteActionIriOrLabel> for &str {
	fn eq(&self, other: &QuoteActionIriOrLabel) -> bool {
		*self == QuoteActionIri || *self == QUOTE_ACTION_LABEL
	}
}
