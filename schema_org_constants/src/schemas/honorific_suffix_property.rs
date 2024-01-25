/// <https://schema.org/honorificSuffix>
pub const HONORIFIC_SUFFIX_PROPERTY_IRI_HTTP: &str = "http://schema.org/honorificSuffix";
/// <https://schema.org/honorificSuffix>
pub const HONORIFIC_SUFFIX_PROPERTY_IRI_HTTPS: &str = "https://schema.org/honorificSuffix";
/// <https://schema.org/honorificSuffix>
pub const HONORIFIC_SUFFIX_PROPERTY_LABEL: &str = "honorificSuffix";
pub struct HonorificSuffixPropertyIri;
impl PartialEq<&str> for HonorificSuffixPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HONORIFIC_SUFFIX_PROPERTY_IRI_HTTP
			|| *other == HONORIFIC_SUFFIX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HonorificSuffixPropertyIri> for &str {
	fn eq(&self, other: &HonorificSuffixPropertyIri) -> bool {
		*self == HONORIFIC_SUFFIX_PROPERTY_IRI_HTTP || *self == HONORIFIC_SUFFIX_PROPERTY_IRI_HTTPS
	}
}
pub struct HonorificSuffixPropertyIriOrLabel;
impl PartialEq<&str> for HonorificSuffixPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HonorificSuffixPropertyIri || *other == HONORIFIC_SUFFIX_PROPERTY_LABEL
	}
}
impl PartialEq<HonorificSuffixPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HonorificSuffixPropertyIriOrLabel) -> bool {
		*self == HonorificSuffixPropertyIri || *self == HONORIFIC_SUFFIX_PROPERTY_LABEL
	}
}
