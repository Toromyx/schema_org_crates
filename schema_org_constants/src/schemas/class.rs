/// <https://schema.org/Class>
pub const CLASS_IRI_HTTP: &str = "http://schema.org/Class";
/// <https://schema.org/Class>
pub const CLASS_IRI_HTTPS: &str = "https://schema.org/Class";
/// <https://schema.org/Class>
pub const CLASS_LABEL: &str = "Class";
pub struct ClassIri;
impl PartialEq<&str> for ClassIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLASS_IRI_HTTP || *other == CLASS_IRI_HTTPS
	}
}
impl PartialEq<ClassIri> for &str {
	fn eq(&self, other: &ClassIri) -> bool {
		*self == CLASS_IRI_HTTP || *self == CLASS_IRI_HTTPS
	}
}
pub struct ClassIriOrLabel;
impl PartialEq<&str> for ClassIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClassIri || *other == CLASS_LABEL
	}
}
impl PartialEq<ClassIriOrLabel> for &str {
	fn eq(&self, other: &ClassIriOrLabel) -> bool {
		*self == ClassIri || *self == CLASS_LABEL
	}
}
