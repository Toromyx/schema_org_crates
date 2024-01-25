/// <https://schema.org/parentService>
pub const PARENT_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/parentService";
/// <https://schema.org/parentService>
pub const PARENT_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parentService";
/// <https://schema.org/parentService>
pub const PARENT_SERVICE_PROPERTY_LABEL: &str = "parentService";
pub struct ParentServicePropertyIri;
impl PartialEq<&str> for ParentServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_SERVICE_PROPERTY_IRI_HTTP || *other == PARENT_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentServicePropertyIri> for &str {
	fn eq(&self, other: &ParentServicePropertyIri) -> bool {
		*self == PARENT_SERVICE_PROPERTY_IRI_HTTP || *self == PARENT_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentServicePropertyIriOrLabel;
impl PartialEq<&str> for ParentServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentServicePropertyIri || *other == PARENT_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<ParentServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentServicePropertyIriOrLabel) -> bool {
		*self == ParentServicePropertyIri || *self == PARENT_SERVICE_PROPERTY_LABEL
	}
}
