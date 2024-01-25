/// <https://schema.org/RoofingContractor>
pub const ROOFING_CONTRACTOR_IRI_HTTP: &str = "http://schema.org/RoofingContractor";
/// <https://schema.org/RoofingContractor>
pub const ROOFING_CONTRACTOR_IRI_HTTPS: &str = "https://schema.org/RoofingContractor";
/// <https://schema.org/RoofingContractor>
pub const ROOFING_CONTRACTOR_LABEL: &str = "RoofingContractor";
pub struct RoofingContractorIri;
impl PartialEq<&str> for RoofingContractorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ROOFING_CONTRACTOR_IRI_HTTP || *other == ROOFING_CONTRACTOR_IRI_HTTPS
	}
}
impl PartialEq<RoofingContractorIri> for &str {
	fn eq(&self, other: &RoofingContractorIri) -> bool {
		*self == ROOFING_CONTRACTOR_IRI_HTTP || *self == ROOFING_CONTRACTOR_IRI_HTTPS
	}
}
pub struct RoofingContractorIriOrLabel;
impl PartialEq<&str> for RoofingContractorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RoofingContractorIri || *other == ROOFING_CONTRACTOR_LABEL
	}
}
impl PartialEq<RoofingContractorIriOrLabel> for &str {
	fn eq(&self, other: &RoofingContractorIriOrLabel) -> bool {
		*self == RoofingContractorIri || *self == ROOFING_CONTRACTOR_LABEL
	}
}
