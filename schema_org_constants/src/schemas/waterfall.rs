/// <https://schema.org/Waterfall>
pub const WATERFALL_IRI_HTTP: &str = "http://schema.org/Waterfall";
/// <https://schema.org/Waterfall>
pub const WATERFALL_IRI_HTTPS: &str = "https://schema.org/Waterfall";
/// <https://schema.org/Waterfall>
pub const WATERFALL_LABEL: &str = "Waterfall";
pub struct WaterfallIri;
impl PartialEq<&str> for WaterfallIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WATERFALL_IRI_HTTP || *other == WATERFALL_IRI_HTTPS
	}
}
impl PartialEq<WaterfallIri> for &str {
	fn eq(&self, other: &WaterfallIri) -> bool {
		*self == WATERFALL_IRI_HTTP || *self == WATERFALL_IRI_HTTPS
	}
}
pub struct WaterfallIriOrLabel;
impl PartialEq<&str> for WaterfallIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WaterfallIri || *other == WATERFALL_LABEL
	}
}
impl PartialEq<WaterfallIriOrLabel> for &str {
	fn eq(&self, other: &WaterfallIriOrLabel) -> bool {
		*self == WaterfallIri || *self == WATERFALL_LABEL
	}
}
