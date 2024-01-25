/// <https://schema.org/object>
pub const OBJECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/object";
/// <https://schema.org/object>
pub const OBJECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/object";
/// <https://schema.org/object>
pub const OBJECT_PROPERTY_LABEL: &str = "object";
pub struct ObjectPropertyIri;
impl PartialEq<&str> for ObjectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBJECT_PROPERTY_IRI_HTTP || *other == OBJECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ObjectPropertyIri> for &str {
	fn eq(&self, other: &ObjectPropertyIri) -> bool {
		*self == OBJECT_PROPERTY_IRI_HTTP || *self == OBJECT_PROPERTY_IRI_HTTPS
	}
}
pub struct ObjectPropertyIriOrLabel;
impl PartialEq<&str> for ObjectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObjectPropertyIri || *other == OBJECT_PROPERTY_LABEL
	}
}
impl PartialEq<ObjectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ObjectPropertyIriOrLabel) -> bool {
		*self == ObjectPropertyIri || *self == OBJECT_PROPERTY_LABEL
	}
}
