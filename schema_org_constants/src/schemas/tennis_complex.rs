/// <https://schema.org/TennisComplex>
pub const TENNIS_COMPLEX_IRI_HTTP: &str = "http://schema.org/TennisComplex";
/// <https://schema.org/TennisComplex>
pub const TENNIS_COMPLEX_IRI_HTTPS: &str = "https://schema.org/TennisComplex";
/// <https://schema.org/TennisComplex>
pub const TENNIS_COMPLEX_LABEL: &str = "TennisComplex";
pub struct TennisComplexIri;
impl PartialEq<&str> for TennisComplexIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TENNIS_COMPLEX_IRI_HTTP || *other == TENNIS_COMPLEX_IRI_HTTPS
	}
}
impl PartialEq<TennisComplexIri> for &str {
	fn eq(&self, other: &TennisComplexIri) -> bool {
		*self == TENNIS_COMPLEX_IRI_HTTP || *self == TENNIS_COMPLEX_IRI_HTTPS
	}
}
pub struct TennisComplexIriOrLabel;
impl PartialEq<&str> for TennisComplexIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TennisComplexIri || *other == TENNIS_COMPLEX_LABEL
	}
}
impl PartialEq<TennisComplexIriOrLabel> for &str {
	fn eq(&self, other: &TennisComplexIriOrLabel) -> bool {
		*self == TennisComplexIri || *self == TENNIS_COMPLEX_LABEL
	}
}
