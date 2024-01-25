/// <https://schema.org/NLNonprofitType>
pub const NL_NONPROFIT_TYPE_IRI_HTTP: &str = "http://schema.org/NLNonprofitType";
/// <https://schema.org/NLNonprofitType>
pub const NL_NONPROFIT_TYPE_IRI_HTTPS: &str = "https://schema.org/NLNonprofitType";
/// <https://schema.org/NLNonprofitType>
pub const NL_NONPROFIT_TYPE_LABEL: &str = "NLNonprofitType";
pub struct NlNonprofitTypeIri;
impl PartialEq<&str> for NlNonprofitTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NL_NONPROFIT_TYPE_IRI_HTTP || *other == NL_NONPROFIT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<NlNonprofitTypeIri> for &str {
	fn eq(&self, other: &NlNonprofitTypeIri) -> bool {
		*self == NL_NONPROFIT_TYPE_IRI_HTTP || *self == NL_NONPROFIT_TYPE_IRI_HTTPS
	}
}
pub struct NlNonprofitTypeIriOrLabel;
impl PartialEq<&str> for NlNonprofitTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NlNonprofitTypeIri || *other == NL_NONPROFIT_TYPE_LABEL
	}
}
impl PartialEq<NlNonprofitTypeIriOrLabel> for &str {
	fn eq(&self, other: &NlNonprofitTypeIriOrLabel) -> bool {
		*self == NlNonprofitTypeIri || *self == NL_NONPROFIT_TYPE_LABEL
	}
}
