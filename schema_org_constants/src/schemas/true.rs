/// <https://schema.org/True>
pub const TRUE_IRI_HTTP: &str = "http://schema.org/True";
/// <https://schema.org/True>
pub const TRUE_IRI_HTTPS: &str = "https://schema.org/True";
/// <https://schema.org/True>
pub const TRUE_LABEL: &str = "True";
pub struct TrueIri;
impl PartialEq<&str> for TrueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRUE_IRI_HTTP || *other == TRUE_IRI_HTTPS
	}
}
impl PartialEq<TrueIri> for &str {
	fn eq(&self, other: &TrueIri) -> bool {
		*self == TRUE_IRI_HTTP || *self == TRUE_IRI_HTTPS
	}
}
pub struct TrueIriOrLabel;
impl PartialEq<&str> for TrueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrueIri || *other == TRUE_LABEL
	}
}
impl PartialEq<TrueIriOrLabel> for &str {
	fn eq(&self, other: &TrueIriOrLabel) -> bool {
		*self == TrueIri || *self == TRUE_LABEL
	}
}
