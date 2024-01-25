/// <https://schema.org/ApprovedIndication>
pub const APPROVED_INDICATION_IRI_HTTP: &str = "http://schema.org/ApprovedIndication";
/// <https://schema.org/ApprovedIndication>
pub const APPROVED_INDICATION_IRI_HTTPS: &str = "https://schema.org/ApprovedIndication";
/// <https://schema.org/ApprovedIndication>
pub const APPROVED_INDICATION_LABEL: &str = "ApprovedIndication";
pub struct ApprovedIndicationIri;
impl PartialEq<&str> for ApprovedIndicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPROVED_INDICATION_IRI_HTTP || *other == APPROVED_INDICATION_IRI_HTTPS
	}
}
impl PartialEq<ApprovedIndicationIri> for &str {
	fn eq(&self, other: &ApprovedIndicationIri) -> bool {
		*self == APPROVED_INDICATION_IRI_HTTP || *self == APPROVED_INDICATION_IRI_HTTPS
	}
}
pub struct ApprovedIndicationIriOrLabel;
impl PartialEq<&str> for ApprovedIndicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApprovedIndicationIri || *other == APPROVED_INDICATION_LABEL
	}
}
impl PartialEq<ApprovedIndicationIriOrLabel> for &str {
	fn eq(&self, other: &ApprovedIndicationIriOrLabel) -> bool {
		*self == ApprovedIndicationIri || *self == APPROVED_INDICATION_LABEL
	}
}
