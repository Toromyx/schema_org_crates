/// <https://schema.org/isUnlabelledFallback>
pub const IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTP: &str = "http://schema.org/isUnlabelledFallback";
/// <https://schema.org/isUnlabelledFallback>
pub const IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isUnlabelledFallback";
/// <https://schema.org/isUnlabelledFallback>
pub const IS_UNLABELLED_FALLBACK_PROPERTY_LABEL: &str = "isUnlabelledFallback";
pub struct IsUnlabelledFallbackPropertyIri;
impl PartialEq<&str> for IsUnlabelledFallbackPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTP
			|| *other == IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsUnlabelledFallbackPropertyIri> for &str {
	fn eq(&self, other: &IsUnlabelledFallbackPropertyIri) -> bool {
		*self == IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTP
			|| *self == IS_UNLABELLED_FALLBACK_PROPERTY_IRI_HTTPS
	}
}
pub struct IsUnlabelledFallbackPropertyIriOrLabel;
impl PartialEq<&str> for IsUnlabelledFallbackPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsUnlabelledFallbackPropertyIri || *other == IS_UNLABELLED_FALLBACK_PROPERTY_LABEL
	}
}
impl PartialEq<IsUnlabelledFallbackPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsUnlabelledFallbackPropertyIriOrLabel) -> bool {
		*self == IsUnlabelledFallbackPropertyIri || *self == IS_UNLABELLED_FALLBACK_PROPERTY_LABEL
	}
}
