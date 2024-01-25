/// <https://schema.org/significance>
pub const SIGNIFICANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/significance";
/// <https://schema.org/significance>
pub const SIGNIFICANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/significance";
/// <https://schema.org/significance>
pub const SIGNIFICANCE_PROPERTY_LABEL: &str = "significance";
pub struct SignificancePropertyIri;
impl PartialEq<&str> for SignificancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIGNIFICANCE_PROPERTY_IRI_HTTP || *other == SIGNIFICANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SignificancePropertyIri> for &str {
	fn eq(&self, other: &SignificancePropertyIri) -> bool {
		*self == SIGNIFICANCE_PROPERTY_IRI_HTTP || *self == SIGNIFICANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct SignificancePropertyIriOrLabel;
impl PartialEq<&str> for SignificancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SignificancePropertyIri || *other == SIGNIFICANCE_PROPERTY_LABEL
	}
}
impl PartialEq<SignificancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SignificancePropertyIriOrLabel) -> bool {
		*self == SignificancePropertyIri || *self == SIGNIFICANCE_PROPERTY_LABEL
	}
}
