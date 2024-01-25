/// <https://schema.org/diagram>
pub const DIAGRAM_PROPERTY_IRI_HTTP: &str = "http://schema.org/diagram";
/// <https://schema.org/diagram>
pub const DIAGRAM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/diagram";
/// <https://schema.org/diagram>
pub const DIAGRAM_PROPERTY_LABEL: &str = "diagram";
pub struct DiagramPropertyIri;
impl PartialEq<&str> for DiagramPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIAGRAM_PROPERTY_IRI_HTTP || *other == DIAGRAM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiagramPropertyIri> for &str {
	fn eq(&self, other: &DiagramPropertyIri) -> bool {
		*self == DIAGRAM_PROPERTY_IRI_HTTP || *self == DIAGRAM_PROPERTY_IRI_HTTPS
	}
}
pub struct DiagramPropertyIriOrLabel;
impl PartialEq<&str> for DiagramPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiagramPropertyIri || *other == DIAGRAM_PROPERTY_LABEL
	}
}
impl PartialEq<DiagramPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiagramPropertyIriOrLabel) -> bool {
		*self == DiagramPropertyIri || *self == DIAGRAM_PROPERTY_LABEL
	}
}
