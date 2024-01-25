/// <https://schema.org/editor>
pub const EDITOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/editor";
/// <https://schema.org/editor>
pub const EDITOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/editor";
/// <https://schema.org/editor>
pub const EDITOR_PROPERTY_LABEL: &str = "editor";
pub struct EditorPropertyIri;
impl PartialEq<&str> for EditorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDITOR_PROPERTY_IRI_HTTP || *other == EDITOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EditorPropertyIri> for &str {
	fn eq(&self, other: &EditorPropertyIri) -> bool {
		*self == EDITOR_PROPERTY_IRI_HTTP || *self == EDITOR_PROPERTY_IRI_HTTPS
	}
}
pub struct EditorPropertyIriOrLabel;
impl PartialEq<&str> for EditorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EditorPropertyIri || *other == EDITOR_PROPERTY_LABEL
	}
}
impl PartialEq<EditorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EditorPropertyIriOrLabel) -> bool {
		*self == EditorPropertyIri || *self == EDITOR_PROPERTY_LABEL
	}
}
