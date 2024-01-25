/// <https://schema.org/model>
pub const MODEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/model";
/// <https://schema.org/model>
pub const MODEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/model";
/// <https://schema.org/model>
pub const MODEL_PROPERTY_LABEL: &str = "model";
pub struct ModelPropertyIri;
impl PartialEq<&str> for ModelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MODEL_PROPERTY_IRI_HTTP || *other == MODEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ModelPropertyIri> for &str {
	fn eq(&self, other: &ModelPropertyIri) -> bool {
		*self == MODEL_PROPERTY_IRI_HTTP || *self == MODEL_PROPERTY_IRI_HTTPS
	}
}
pub struct ModelPropertyIriOrLabel;
impl PartialEq<&str> for ModelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ModelPropertyIri || *other == MODEL_PROPERTY_LABEL
	}
}
impl PartialEq<ModelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ModelPropertyIriOrLabel) -> bool {
		*self == ModelPropertyIri || *self == MODEL_PROPERTY_LABEL
	}
}
