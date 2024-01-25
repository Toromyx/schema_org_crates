/// <https://schema.org/honorificPrefix>
pub const HONORIFIC_PREFIX_PROPERTY_IRI_HTTP: &str = "http://schema.org/honorificPrefix";
/// <https://schema.org/honorificPrefix>
pub const HONORIFIC_PREFIX_PROPERTY_IRI_HTTPS: &str = "https://schema.org/honorificPrefix";
/// <https://schema.org/honorificPrefix>
pub const HONORIFIC_PREFIX_PROPERTY_LABEL: &str = "honorificPrefix";
pub struct HonorificPrefixPropertyIri;
impl PartialEq<&str> for HonorificPrefixPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HONORIFIC_PREFIX_PROPERTY_IRI_HTTP
			|| *other == HONORIFIC_PREFIX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HonorificPrefixPropertyIri> for &str {
	fn eq(&self, other: &HonorificPrefixPropertyIri) -> bool {
		*self == HONORIFIC_PREFIX_PROPERTY_IRI_HTTP || *self == HONORIFIC_PREFIX_PROPERTY_IRI_HTTPS
	}
}
pub struct HonorificPrefixPropertyIriOrLabel;
impl PartialEq<&str> for HonorificPrefixPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HonorificPrefixPropertyIri || *other == HONORIFIC_PREFIX_PROPERTY_LABEL
	}
}
impl PartialEq<HonorificPrefixPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HonorificPrefixPropertyIriOrLabel) -> bool {
		*self == HonorificPrefixPropertyIri || *self == HONORIFIC_PREFIX_PROPERTY_LABEL
	}
}
