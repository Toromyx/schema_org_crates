/// <https://schema.org/programmingModel>
pub const PROGRAMMING_MODEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/programmingModel";
/// <https://schema.org/programmingModel>
pub const PROGRAMMING_MODEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/programmingModel";
/// <https://schema.org/programmingModel>
pub const PROGRAMMING_MODEL_PROPERTY_LABEL: &str = "programmingModel";
pub struct ProgrammingModelPropertyIri;
impl PartialEq<&str> for ProgrammingModelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAMMING_MODEL_PROPERTY_IRI_HTTP
			|| *other == PROGRAMMING_MODEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgrammingModelPropertyIri> for &str {
	fn eq(&self, other: &ProgrammingModelPropertyIri) -> bool {
		*self == PROGRAMMING_MODEL_PROPERTY_IRI_HTTP
			|| *self == PROGRAMMING_MODEL_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgrammingModelPropertyIriOrLabel;
impl PartialEq<&str> for ProgrammingModelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgrammingModelPropertyIri || *other == PROGRAMMING_MODEL_PROPERTY_LABEL
	}
}
impl PartialEq<ProgrammingModelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgrammingModelPropertyIriOrLabel) -> bool {
		*self == ProgrammingModelPropertyIri || *self == PROGRAMMING_MODEL_PROPERTY_LABEL
	}
}
