/// <https://schema.org/GeneralContractor>
pub const GENERAL_CONTRACTOR_IRI_HTTP: &str = "http://schema.org/GeneralContractor";
/// <https://schema.org/GeneralContractor>
pub const GENERAL_CONTRACTOR_IRI_HTTPS: &str = "https://schema.org/GeneralContractor";
/// <https://schema.org/GeneralContractor>
pub const GENERAL_CONTRACTOR_LABEL: &str = "GeneralContractor";
pub struct GeneralContractorIri;
impl PartialEq<&str> for GeneralContractorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENERAL_CONTRACTOR_IRI_HTTP || *other == GENERAL_CONTRACTOR_IRI_HTTPS
	}
}
impl PartialEq<GeneralContractorIri> for &str {
	fn eq(&self, other: &GeneralContractorIri) -> bool {
		*self == GENERAL_CONTRACTOR_IRI_HTTP || *self == GENERAL_CONTRACTOR_IRI_HTTPS
	}
}
pub struct GeneralContractorIriOrLabel;
impl PartialEq<&str> for GeneralContractorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeneralContractorIri || *other == GENERAL_CONTRACTOR_LABEL
	}
}
impl PartialEq<GeneralContractorIriOrLabel> for &str {
	fn eq(&self, other: &GeneralContractorIriOrLabel) -> bool {
		*self == GeneralContractorIri || *self == GENERAL_CONTRACTOR_LABEL
	}
}
