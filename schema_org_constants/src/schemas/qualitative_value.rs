/// <https://schema.org/QualitativeValue>
pub const QUALITATIVE_VALUE_IRI_HTTP: &str = "http://schema.org/QualitativeValue";
/// <https://schema.org/QualitativeValue>
pub const QUALITATIVE_VALUE_IRI_HTTPS: &str = "https://schema.org/QualitativeValue";
/// <https://schema.org/QualitativeValue>
pub const QUALITATIVE_VALUE_LABEL: &str = "QualitativeValue";
pub struct QualitativeValueIri;
impl PartialEq<&str> for QualitativeValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUALITATIVE_VALUE_IRI_HTTP || *other == QUALITATIVE_VALUE_IRI_HTTPS
	}
}
impl PartialEq<QualitativeValueIri> for &str {
	fn eq(&self, other: &QualitativeValueIri) -> bool {
		*self == QUALITATIVE_VALUE_IRI_HTTP || *self == QUALITATIVE_VALUE_IRI_HTTPS
	}
}
pub struct QualitativeValueIriOrLabel;
impl PartialEq<&str> for QualitativeValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QualitativeValueIri || *other == QUALITATIVE_VALUE_LABEL
	}
}
impl PartialEq<QualitativeValueIriOrLabel> for &str {
	fn eq(&self, other: &QualitativeValueIriOrLabel) -> bool {
		*self == QualitativeValueIri || *self == QUALITATIVE_VALUE_LABEL
	}
}
