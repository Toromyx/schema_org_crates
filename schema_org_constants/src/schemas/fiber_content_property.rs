/// <https://schema.org/fiberContent>
pub const FIBER_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/fiberContent";
/// <https://schema.org/fiberContent>
pub const FIBER_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fiberContent";
/// <https://schema.org/fiberContent>
pub const FIBER_CONTENT_PROPERTY_LABEL: &str = "fiberContent";
pub struct FiberContentPropertyIri;
impl PartialEq<&str> for FiberContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FIBER_CONTENT_PROPERTY_IRI_HTTP || *other == FIBER_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FiberContentPropertyIri> for &str {
	fn eq(&self, other: &FiberContentPropertyIri) -> bool {
		*self == FIBER_CONTENT_PROPERTY_IRI_HTTP || *self == FIBER_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct FiberContentPropertyIriOrLabel;
impl PartialEq<&str> for FiberContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FiberContentPropertyIri || *other == FIBER_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<FiberContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FiberContentPropertyIriOrLabel) -> bool {
		*self == FiberContentPropertyIri || *self == FIBER_CONTENT_PROPERTY_LABEL
	}
}
