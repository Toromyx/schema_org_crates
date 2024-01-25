/// <https://schema.org/SizeSpecification>
pub const SIZE_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/SizeSpecification";
/// <https://schema.org/SizeSpecification>
pub const SIZE_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/SizeSpecification";
/// <https://schema.org/SizeSpecification>
pub const SIZE_SPECIFICATION_LABEL: &str = "SizeSpecification";
pub struct SizeSpecificationIri;
impl PartialEq<&str> for SizeSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_SPECIFICATION_IRI_HTTP || *other == SIZE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<SizeSpecificationIri> for &str {
	fn eq(&self, other: &SizeSpecificationIri) -> bool {
		*self == SIZE_SPECIFICATION_IRI_HTTP || *self == SIZE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct SizeSpecificationIriOrLabel;
impl PartialEq<&str> for SizeSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeSpecificationIri || *other == SIZE_SPECIFICATION_LABEL
	}
}
impl PartialEq<SizeSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &SizeSpecificationIriOrLabel) -> bool {
		*self == SizeSpecificationIri || *self == SIZE_SPECIFICATION_LABEL
	}
}
