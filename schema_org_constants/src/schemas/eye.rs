/// <https://schema.org/Eye>
pub const EYE_IRI_HTTP: &str = "http://schema.org/Eye";
/// <https://schema.org/Eye>
pub const EYE_IRI_HTTPS: &str = "https://schema.org/Eye";
/// <https://schema.org/Eye>
pub const EYE_LABEL: &str = "Eye";
pub struct EyeIri;
impl PartialEq<&str> for EyeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EYE_IRI_HTTP || *other == EYE_IRI_HTTPS
	}
}
impl PartialEq<EyeIri> for &str {
	fn eq(&self, other: &EyeIri) -> bool {
		*self == EYE_IRI_HTTP || *self == EYE_IRI_HTTPS
	}
}
pub struct EyeIriOrLabel;
impl PartialEq<&str> for EyeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EyeIri || *other == EYE_LABEL
	}
}
impl PartialEq<EyeIriOrLabel> for &str {
	fn eq(&self, other: &EyeIriOrLabel) -> bool {
		*self == EyeIri || *self == EYE_LABEL
	}
}
