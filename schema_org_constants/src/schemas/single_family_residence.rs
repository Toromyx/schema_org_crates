/// <https://schema.org/SingleFamilyResidence>
pub const SINGLE_FAMILY_RESIDENCE_IRI_HTTP: &str = "http://schema.org/SingleFamilyResidence";
/// <https://schema.org/SingleFamilyResidence>
pub const SINGLE_FAMILY_RESIDENCE_IRI_HTTPS: &str = "https://schema.org/SingleFamilyResidence";
/// <https://schema.org/SingleFamilyResidence>
pub const SINGLE_FAMILY_RESIDENCE_LABEL: &str = "SingleFamilyResidence";
pub struct SingleFamilyResidenceIri;
impl PartialEq<&str> for SingleFamilyResidenceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SINGLE_FAMILY_RESIDENCE_IRI_HTTP || *other == SINGLE_FAMILY_RESIDENCE_IRI_HTTPS
	}
}
impl PartialEq<SingleFamilyResidenceIri> for &str {
	fn eq(&self, other: &SingleFamilyResidenceIri) -> bool {
		*self == SINGLE_FAMILY_RESIDENCE_IRI_HTTP || *self == SINGLE_FAMILY_RESIDENCE_IRI_HTTPS
	}
}
pub struct SingleFamilyResidenceIriOrLabel;
impl PartialEq<&str> for SingleFamilyResidenceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SingleFamilyResidenceIri || *other == SINGLE_FAMILY_RESIDENCE_LABEL
	}
}
impl PartialEq<SingleFamilyResidenceIriOrLabel> for &str {
	fn eq(&self, other: &SingleFamilyResidenceIriOrLabel) -> bool {
		*self == SingleFamilyResidenceIri || *self == SINGLE_FAMILY_RESIDENCE_LABEL
	}
}
