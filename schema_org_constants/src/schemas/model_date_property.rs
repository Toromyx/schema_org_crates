/// <https://schema.org/modelDate>
pub const MODEL_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/modelDate";
/// <https://schema.org/modelDate>
pub const MODEL_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/modelDate";
/// <https://schema.org/modelDate>
pub const MODEL_DATE_PROPERTY_LABEL: &str = "modelDate";
pub struct ModelDatePropertyIri;
impl PartialEq<&str> for ModelDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MODEL_DATE_PROPERTY_IRI_HTTP || *other == MODEL_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ModelDatePropertyIri> for &str {
	fn eq(&self, other: &ModelDatePropertyIri) -> bool {
		*self == MODEL_DATE_PROPERTY_IRI_HTTP || *self == MODEL_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ModelDatePropertyIriOrLabel;
impl PartialEq<&str> for ModelDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ModelDatePropertyIri || *other == MODEL_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ModelDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ModelDatePropertyIriOrLabel) -> bool {
		*self == ModelDatePropertyIri || *self == MODEL_DATE_PROPERTY_LABEL
	}
}
