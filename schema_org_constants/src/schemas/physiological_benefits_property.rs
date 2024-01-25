/// <https://schema.org/physiologicalBenefits>
pub const PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/physiologicalBenefits";
/// <https://schema.org/physiologicalBenefits>
pub const PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/physiologicalBenefits";
/// <https://schema.org/physiologicalBenefits>
pub const PHYSIOLOGICAL_BENEFITS_PROPERTY_LABEL: &str = "physiologicalBenefits";
pub struct PhysiologicalBenefitsPropertyIri;
impl PartialEq<&str> for PhysiologicalBenefitsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTP
			|| *other == PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PhysiologicalBenefitsPropertyIri> for &str {
	fn eq(&self, other: &PhysiologicalBenefitsPropertyIri) -> bool {
		*self == PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTP
			|| *self == PHYSIOLOGICAL_BENEFITS_PROPERTY_IRI_HTTPS
	}
}
pub struct PhysiologicalBenefitsPropertyIriOrLabel;
impl PartialEq<&str> for PhysiologicalBenefitsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysiologicalBenefitsPropertyIri
			|| *other == PHYSIOLOGICAL_BENEFITS_PROPERTY_LABEL
	}
}
impl PartialEq<PhysiologicalBenefitsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PhysiologicalBenefitsPropertyIriOrLabel) -> bool {
		*self == PhysiologicalBenefitsPropertyIri || *self == PHYSIOLOGICAL_BENEFITS_PROPERTY_LABEL
	}
}
