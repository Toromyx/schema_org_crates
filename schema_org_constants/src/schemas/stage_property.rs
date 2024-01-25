/// <https://schema.org/stage>
pub const STAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/stage";
/// <https://schema.org/stage>
pub const STAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/stage";
/// <https://schema.org/stage>
pub const STAGE_PROPERTY_LABEL: &str = "stage";
pub struct StagePropertyIri;
impl PartialEq<&str> for StagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAGE_PROPERTY_IRI_HTTP || *other == STAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StagePropertyIri> for &str {
	fn eq(&self, other: &StagePropertyIri) -> bool {
		*self == STAGE_PROPERTY_IRI_HTTP || *self == STAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct StagePropertyIriOrLabel;
impl PartialEq<&str> for StagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StagePropertyIri || *other == STAGE_PROPERTY_LABEL
	}
}
impl PartialEq<StagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StagePropertyIriOrLabel) -> bool {
		*self == StagePropertyIri || *self == STAGE_PROPERTY_LABEL
	}
}
