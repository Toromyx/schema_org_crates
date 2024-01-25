/// <https://schema.org/inker>
pub const INKER_PROPERTY_IRI_HTTP: &str = "http://schema.org/inker";
/// <https://schema.org/inker>
pub const INKER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inker";
/// <https://schema.org/inker>
pub const INKER_PROPERTY_LABEL: &str = "inker";
pub struct InkerPropertyIri;
impl PartialEq<&str> for InkerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INKER_PROPERTY_IRI_HTTP || *other == INKER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InkerPropertyIri> for &str {
	fn eq(&self, other: &InkerPropertyIri) -> bool {
		*self == INKER_PROPERTY_IRI_HTTP || *self == INKER_PROPERTY_IRI_HTTPS
	}
}
pub struct InkerPropertyIriOrLabel;
impl PartialEq<&str> for InkerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InkerPropertyIri || *other == INKER_PROPERTY_LABEL
	}
}
impl PartialEq<InkerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InkerPropertyIriOrLabel) -> bool {
		*self == InkerPropertyIri || *self == INKER_PROPERTY_LABEL
	}
}
