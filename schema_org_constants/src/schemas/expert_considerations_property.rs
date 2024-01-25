/// <https://schema.org/expertConsiderations>
pub const EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/expertConsiderations";
/// <https://schema.org/expertConsiderations>
pub const EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/expertConsiderations";
/// <https://schema.org/expertConsiderations>
pub const EXPERT_CONSIDERATIONS_PROPERTY_LABEL: &str = "expertConsiderations";
pub struct ExpertConsiderationsPropertyIri;
impl PartialEq<&str> for ExpertConsiderationsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTP
			|| *other == EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpertConsiderationsPropertyIri> for &str {
	fn eq(&self, other: &ExpertConsiderationsPropertyIri) -> bool {
		*self == EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTP
			|| *self == EXPERT_CONSIDERATIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpertConsiderationsPropertyIriOrLabel;
impl PartialEq<&str> for ExpertConsiderationsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpertConsiderationsPropertyIri || *other == EXPERT_CONSIDERATIONS_PROPERTY_LABEL
	}
}
impl PartialEq<ExpertConsiderationsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpertConsiderationsPropertyIriOrLabel) -> bool {
		*self == ExpertConsiderationsPropertyIri || *self == EXPERT_CONSIDERATIONS_PROPERTY_LABEL
	}
}
