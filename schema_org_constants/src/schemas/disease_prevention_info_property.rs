/// <https://schema.org/diseasePreventionInfo>
pub const DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/diseasePreventionInfo";
/// <https://schema.org/diseasePreventionInfo>
pub const DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/diseasePreventionInfo";
/// <https://schema.org/diseasePreventionInfo>
pub const DISEASE_PREVENTION_INFO_PROPERTY_LABEL: &str = "diseasePreventionInfo";
pub struct DiseasePreventionInfoPropertyIri;
impl PartialEq<&str> for DiseasePreventionInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTP
			|| *other == DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiseasePreventionInfoPropertyIri> for &str {
	fn eq(&self, other: &DiseasePreventionInfoPropertyIri) -> bool {
		*self == DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTP
			|| *self == DISEASE_PREVENTION_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct DiseasePreventionInfoPropertyIriOrLabel;
impl PartialEq<&str> for DiseasePreventionInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiseasePreventionInfoPropertyIri
			|| *other == DISEASE_PREVENTION_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<DiseasePreventionInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiseasePreventionInfoPropertyIriOrLabel) -> bool {
		*self == DiseasePreventionInfoPropertyIri || *self == DISEASE_PREVENTION_INFO_PROPERTY_LABEL
	}
}
