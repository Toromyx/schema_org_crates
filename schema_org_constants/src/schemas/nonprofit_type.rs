/// <https://schema.org/NonprofitType>
pub const NONPROFIT_TYPE_IRI_HTTP: &str = "http://schema.org/NonprofitType";
/// <https://schema.org/NonprofitType>
pub const NONPROFIT_TYPE_IRI_HTTPS: &str = "https://schema.org/NonprofitType";
/// <https://schema.org/NonprofitType>
pub const NONPROFIT_TYPE_LABEL: &str = "NonprofitType";
pub struct NonprofitTypeIri;
impl PartialEq<&str> for NonprofitTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_TYPE_IRI_HTTP || *other == NONPROFIT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<NonprofitTypeIri> for &str {
	fn eq(&self, other: &NonprofitTypeIri) -> bool {
		*self == NONPROFIT_TYPE_IRI_HTTP || *self == NONPROFIT_TYPE_IRI_HTTPS
	}
}
pub struct NonprofitTypeIriOrLabel;
impl PartialEq<&str> for NonprofitTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonprofitTypeIri || *other == NONPROFIT_TYPE_LABEL
	}
}
impl PartialEq<NonprofitTypeIriOrLabel> for &str {
	fn eq(&self, other: &NonprofitTypeIriOrLabel) -> bool {
		*self == NonprofitTypeIri || *self == NONPROFIT_TYPE_LABEL
	}
}
