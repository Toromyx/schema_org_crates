/// <https://schema.org/Endocrine>
pub const ENDOCRINE_IRI_HTTP: &str = "http://schema.org/Endocrine";
/// <https://schema.org/Endocrine>
pub const ENDOCRINE_IRI_HTTPS: &str = "https://schema.org/Endocrine";
/// <https://schema.org/Endocrine>
pub const ENDOCRINE_LABEL: &str = "Endocrine";
pub struct EndocrineIri;
impl PartialEq<&str> for EndocrineIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENDOCRINE_IRI_HTTP || *other == ENDOCRINE_IRI_HTTPS
	}
}
impl PartialEq<EndocrineIri> for &str {
	fn eq(&self, other: &EndocrineIri) -> bool {
		*self == ENDOCRINE_IRI_HTTP || *self == ENDOCRINE_IRI_HTTPS
	}
}
pub struct EndocrineIriOrLabel;
impl PartialEq<&str> for EndocrineIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndocrineIri || *other == ENDOCRINE_LABEL
	}
}
impl PartialEq<EndocrineIriOrLabel> for &str {
	fn eq(&self, other: &EndocrineIriOrLabel) -> bool {
		*self == EndocrineIri || *self == ENDOCRINE_LABEL
	}
}
