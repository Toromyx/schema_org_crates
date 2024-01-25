/// <https://schema.org/InfectiousDisease>
pub const INFECTIOUS_DISEASE_IRI_HTTP: &str = "http://schema.org/InfectiousDisease";
/// <https://schema.org/InfectiousDisease>
pub const INFECTIOUS_DISEASE_IRI_HTTPS: &str = "https://schema.org/InfectiousDisease";
/// <https://schema.org/InfectiousDisease>
pub const INFECTIOUS_DISEASE_LABEL: &str = "InfectiousDisease";
pub struct InfectiousDiseaseIri;
impl PartialEq<&str> for InfectiousDiseaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFECTIOUS_DISEASE_IRI_HTTP || *other == INFECTIOUS_DISEASE_IRI_HTTPS
	}
}
impl PartialEq<InfectiousDiseaseIri> for &str {
	fn eq(&self, other: &InfectiousDiseaseIri) -> bool {
		*self == INFECTIOUS_DISEASE_IRI_HTTP || *self == INFECTIOUS_DISEASE_IRI_HTTPS
	}
}
pub struct InfectiousDiseaseIriOrLabel;
impl PartialEq<&str> for InfectiousDiseaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InfectiousDiseaseIri || *other == INFECTIOUS_DISEASE_LABEL
	}
}
impl PartialEq<InfectiousDiseaseIriOrLabel> for &str {
	fn eq(&self, other: &InfectiousDiseaseIriOrLabel) -> bool {
		*self == InfectiousDiseaseIri || *self == INFECTIOUS_DISEASE_LABEL
	}
}
