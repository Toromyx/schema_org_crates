/// <https://schema.org/Integer>
pub const INTEGER_IRI_HTTP: &str = "http://schema.org/Integer";
/// <https://schema.org/Integer>
pub const INTEGER_IRI_HTTPS: &str = "https://schema.org/Integer";
/// <https://schema.org/Integer>
pub const INTEGER_LABEL: &str = "Integer";
pub struct IntegerIri;
impl PartialEq<&str> for IntegerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTEGER_IRI_HTTP || *other == INTEGER_IRI_HTTPS
	}
}
impl PartialEq<IntegerIri> for &str {
	fn eq(&self, other: &IntegerIri) -> bool {
		*self == INTEGER_IRI_HTTP || *self == INTEGER_IRI_HTTPS
	}
}
pub struct IntegerIriOrLabel;
impl PartialEq<&str> for IntegerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IntegerIri || *other == INTEGER_LABEL
	}
}
impl PartialEq<IntegerIriOrLabel> for &str {
	fn eq(&self, other: &IntegerIriOrLabel) -> bool {
		*self == IntegerIri || *self == INTEGER_LABEL
	}
}
