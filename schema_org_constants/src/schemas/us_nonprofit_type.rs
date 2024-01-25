/// <https://schema.org/USNonprofitType>
pub const US_NONPROFIT_TYPE_IRI_HTTP: &str = "http://schema.org/USNonprofitType";
/// <https://schema.org/USNonprofitType>
pub const US_NONPROFIT_TYPE_IRI_HTTPS: &str = "https://schema.org/USNonprofitType";
/// <https://schema.org/USNonprofitType>
pub const US_NONPROFIT_TYPE_LABEL: &str = "USNonprofitType";
pub struct UsNonprofitTypeIri;
impl PartialEq<&str> for UsNonprofitTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == US_NONPROFIT_TYPE_IRI_HTTP || *other == US_NONPROFIT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<UsNonprofitTypeIri> for &str {
	fn eq(&self, other: &UsNonprofitTypeIri) -> bool {
		*self == US_NONPROFIT_TYPE_IRI_HTTP || *self == US_NONPROFIT_TYPE_IRI_HTTPS
	}
}
pub struct UsNonprofitTypeIriOrLabel;
impl PartialEq<&str> for UsNonprofitTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsNonprofitTypeIri || *other == US_NONPROFIT_TYPE_LABEL
	}
}
impl PartialEq<UsNonprofitTypeIriOrLabel> for &str {
	fn eq(&self, other: &UsNonprofitTypeIriOrLabel) -> bool {
		*self == UsNonprofitTypeIri || *self == US_NONPROFIT_TYPE_LABEL
	}
}
