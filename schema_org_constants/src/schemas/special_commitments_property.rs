/// <https://schema.org/specialCommitments>
pub const SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/specialCommitments";
/// <https://schema.org/specialCommitments>
pub const SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/specialCommitments";
/// <https://schema.org/specialCommitments>
pub const SPECIAL_COMMITMENTS_PROPERTY_LABEL: &str = "specialCommitments";
pub struct SpecialCommitmentsPropertyIri;
impl PartialEq<&str> for SpecialCommitmentsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTP
			|| *other == SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpecialCommitmentsPropertyIri> for &str {
	fn eq(&self, other: &SpecialCommitmentsPropertyIri) -> bool {
		*self == SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTP
			|| *self == SPECIAL_COMMITMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct SpecialCommitmentsPropertyIriOrLabel;
impl PartialEq<&str> for SpecialCommitmentsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpecialCommitmentsPropertyIri || *other == SPECIAL_COMMITMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<SpecialCommitmentsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpecialCommitmentsPropertyIriOrLabel) -> bool {
		*self == SpecialCommitmentsPropertyIri || *self == SPECIAL_COMMITMENTS_PROPERTY_LABEL
	}
}
