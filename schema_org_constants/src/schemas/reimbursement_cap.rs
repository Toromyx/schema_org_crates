/// <https://schema.org/ReimbursementCap>
pub const REIMBURSEMENT_CAP_IRI_HTTP: &str = "http://schema.org/ReimbursementCap";
/// <https://schema.org/ReimbursementCap>
pub const REIMBURSEMENT_CAP_IRI_HTTPS: &str = "https://schema.org/ReimbursementCap";
/// <https://schema.org/ReimbursementCap>
pub const REIMBURSEMENT_CAP_LABEL: &str = "ReimbursementCap";
pub struct ReimbursementCapIri;
impl PartialEq<&str> for ReimbursementCapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REIMBURSEMENT_CAP_IRI_HTTP || *other == REIMBURSEMENT_CAP_IRI_HTTPS
	}
}
impl PartialEq<ReimbursementCapIri> for &str {
	fn eq(&self, other: &ReimbursementCapIri) -> bool {
		*self == REIMBURSEMENT_CAP_IRI_HTTP || *self == REIMBURSEMENT_CAP_IRI_HTTPS
	}
}
pub struct ReimbursementCapIriOrLabel;
impl PartialEq<&str> for ReimbursementCapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReimbursementCapIri || *other == REIMBURSEMENT_CAP_LABEL
	}
}
impl PartialEq<ReimbursementCapIriOrLabel> for &str {
	fn eq(&self, other: &ReimbursementCapIriOrLabel) -> bool {
		*self == ReimbursementCapIri || *self == REIMBURSEMENT_CAP_LABEL
	}
}
