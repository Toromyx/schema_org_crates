/// <https://schema.org/structuralClass>
pub const STRUCTURAL_CLASS_PROPERTY_IRI_HTTP: &str = "http://schema.org/structuralClass";
/// <https://schema.org/structuralClass>
pub const STRUCTURAL_CLASS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/structuralClass";
/// <https://schema.org/structuralClass>
pub const STRUCTURAL_CLASS_PROPERTY_LABEL: &str = "structuralClass";
pub struct StructuralClassPropertyIri;
impl PartialEq<&str> for StructuralClassPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STRUCTURAL_CLASS_PROPERTY_IRI_HTTP
			|| *other == STRUCTURAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StructuralClassPropertyIri> for &str {
	fn eq(&self, other: &StructuralClassPropertyIri) -> bool {
		*self == STRUCTURAL_CLASS_PROPERTY_IRI_HTTP || *self == STRUCTURAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
pub struct StructuralClassPropertyIriOrLabel;
impl PartialEq<&str> for StructuralClassPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StructuralClassPropertyIri || *other == STRUCTURAL_CLASS_PROPERTY_LABEL
	}
}
impl PartialEq<StructuralClassPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StructuralClassPropertyIriOrLabel) -> bool {
		*self == StructuralClassPropertyIri || *self == STRUCTURAL_CLASS_PROPERTY_LABEL
	}
}
