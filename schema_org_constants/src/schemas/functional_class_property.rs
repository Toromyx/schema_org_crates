/// <https://schema.org/functionalClass>
pub const FUNCTIONAL_CLASS_PROPERTY_IRI_HTTP: &str = "http://schema.org/functionalClass";
/// <https://schema.org/functionalClass>
pub const FUNCTIONAL_CLASS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/functionalClass";
/// <https://schema.org/functionalClass>
pub const FUNCTIONAL_CLASS_PROPERTY_LABEL: &str = "functionalClass";
pub struct FunctionalClassPropertyIri;
impl PartialEq<&str> for FunctionalClassPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNCTIONAL_CLASS_PROPERTY_IRI_HTTP
			|| *other == FUNCTIONAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FunctionalClassPropertyIri> for &str {
	fn eq(&self, other: &FunctionalClassPropertyIri) -> bool {
		*self == FUNCTIONAL_CLASS_PROPERTY_IRI_HTTP || *self == FUNCTIONAL_CLASS_PROPERTY_IRI_HTTPS
	}
}
pub struct FunctionalClassPropertyIriOrLabel;
impl PartialEq<&str> for FunctionalClassPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FunctionalClassPropertyIri || *other == FUNCTIONAL_CLASS_PROPERTY_LABEL
	}
}
impl PartialEq<FunctionalClassPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FunctionalClassPropertyIriOrLabel) -> bool {
		*self == FunctionalClassPropertyIri || *self == FUNCTIONAL_CLASS_PROPERTY_LABEL
	}
}
