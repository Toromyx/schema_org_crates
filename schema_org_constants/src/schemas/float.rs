/// <https://schema.org/Float>
pub const FLOAT_IRI_HTTP: &str = "http://schema.org/Float";
/// <https://schema.org/Float>
pub const FLOAT_IRI_HTTPS: &str = "https://schema.org/Float";
/// <https://schema.org/Float>
pub const FLOAT_LABEL: &str = "Float";
pub struct FloatIri;
impl PartialEq<&str> for FloatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLOAT_IRI_HTTP || *other == FLOAT_IRI_HTTPS
	}
}
impl PartialEq<FloatIri> for &str {
	fn eq(&self, other: &FloatIri) -> bool {
		*self == FLOAT_IRI_HTTP || *self == FLOAT_IRI_HTTPS
	}
}
pub struct FloatIriOrLabel;
impl PartialEq<&str> for FloatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloatIri || *other == FLOAT_LABEL
	}
}
impl PartialEq<FloatIriOrLabel> for &str {
	fn eq(&self, other: &FloatIriOrLabel) -> bool {
		*self == FloatIri || *self == FLOAT_LABEL
	}
}
