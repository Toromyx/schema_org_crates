/// <https://schema.org/NonprofitSBBI>
pub const NONPROFIT_SBBI_IRI_HTTP: &str = "http://schema.org/NonprofitSBBI";
/// <https://schema.org/NonprofitSBBI>
pub const NONPROFIT_SBBI_IRI_HTTPS: &str = "https://schema.org/NonprofitSBBI";
/// <https://schema.org/NonprofitSBBI>
pub const NONPROFIT_SBBI_LABEL: &str = "NonprofitSBBI";
pub struct NonprofitSbbiIri;
impl PartialEq<&str> for NonprofitSbbiIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_SBBI_IRI_HTTP || *other == NONPROFIT_SBBI_IRI_HTTPS
	}
}
impl PartialEq<NonprofitSbbiIri> for &str {
	fn eq(&self, other: &NonprofitSbbiIri) -> bool {
		*self == NONPROFIT_SBBI_IRI_HTTP || *self == NONPROFIT_SBBI_IRI_HTTPS
	}
}
pub struct NonprofitSbbiIriOrLabel;
impl PartialEq<&str> for NonprofitSbbiIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonprofitSbbiIri || *other == NONPROFIT_SBBI_LABEL
	}
}
impl PartialEq<NonprofitSbbiIriOrLabel> for &str {
	fn eq(&self, other: &NonprofitSbbiIriOrLabel) -> bool {
		*self == NonprofitSbbiIri || *self == NONPROFIT_SBBI_LABEL
	}
}
