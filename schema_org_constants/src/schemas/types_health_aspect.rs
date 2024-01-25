/// <https://schema.org/TypesHealthAspect>
pub const TYPES_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/TypesHealthAspect";
/// <https://schema.org/TypesHealthAspect>
pub const TYPES_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/TypesHealthAspect";
/// <https://schema.org/TypesHealthAspect>
pub const TYPES_HEALTH_ASPECT_LABEL: &str = "TypesHealthAspect";
pub struct TypesHealthAspectIri;
impl PartialEq<&str> for TypesHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPES_HEALTH_ASPECT_IRI_HTTP || *other == TYPES_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<TypesHealthAspectIri> for &str {
	fn eq(&self, other: &TypesHealthAspectIri) -> bool {
		*self == TYPES_HEALTH_ASPECT_IRI_HTTP || *self == TYPES_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct TypesHealthAspectIriOrLabel;
impl PartialEq<&str> for TypesHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypesHealthAspectIri || *other == TYPES_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<TypesHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &TypesHealthAspectIriOrLabel) -> bool {
		*self == TypesHealthAspectIri || *self == TYPES_HEALTH_ASPECT_LABEL
	}
}
