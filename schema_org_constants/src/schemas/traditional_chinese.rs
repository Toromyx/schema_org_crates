/// <https://schema.org/TraditionalChinese>
pub const TRADITIONAL_CHINESE_IRI_HTTP: &str = "http://schema.org/TraditionalChinese";
/// <https://schema.org/TraditionalChinese>
pub const TRADITIONAL_CHINESE_IRI_HTTPS: &str = "https://schema.org/TraditionalChinese";
/// <https://schema.org/TraditionalChinese>
pub const TRADITIONAL_CHINESE_LABEL: &str = "TraditionalChinese";
pub struct TraditionalChineseIri;
impl PartialEq<&str> for TraditionalChineseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRADITIONAL_CHINESE_IRI_HTTP || *other == TRADITIONAL_CHINESE_IRI_HTTPS
	}
}
impl PartialEq<TraditionalChineseIri> for &str {
	fn eq(&self, other: &TraditionalChineseIri) -> bool {
		*self == TRADITIONAL_CHINESE_IRI_HTTP || *self == TRADITIONAL_CHINESE_IRI_HTTPS
	}
}
pub struct TraditionalChineseIriOrLabel;
impl PartialEq<&str> for TraditionalChineseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TraditionalChineseIri || *other == TRADITIONAL_CHINESE_LABEL
	}
}
impl PartialEq<TraditionalChineseIriOrLabel> for &str {
	fn eq(&self, other: &TraditionalChineseIriOrLabel) -> bool {
		*self == TraditionalChineseIri || *self == TRADITIONAL_CHINESE_LABEL
	}
}
