/// <https://schema.org/LaboratoryScience>
pub const LABORATORY_SCIENCE_IRI_HTTP: &str = "http://schema.org/LaboratoryScience";
/// <https://schema.org/LaboratoryScience>
pub const LABORATORY_SCIENCE_IRI_HTTPS: &str = "https://schema.org/LaboratoryScience";
/// <https://schema.org/LaboratoryScience>
pub const LABORATORY_SCIENCE_LABEL: &str = "LaboratoryScience";
pub struct LaboratoryScienceIri;
impl PartialEq<&str> for LaboratoryScienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LABORATORY_SCIENCE_IRI_HTTP || *other == LABORATORY_SCIENCE_IRI_HTTPS
	}
}
impl PartialEq<LaboratoryScienceIri> for &str {
	fn eq(&self, other: &LaboratoryScienceIri) -> bool {
		*self == LABORATORY_SCIENCE_IRI_HTTP || *self == LABORATORY_SCIENCE_IRI_HTTPS
	}
}
pub struct LaboratoryScienceIriOrLabel;
impl PartialEq<&str> for LaboratoryScienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LaboratoryScienceIri || *other == LABORATORY_SCIENCE_LABEL
	}
}
impl PartialEq<LaboratoryScienceIriOrLabel> for &str {
	fn eq(&self, other: &LaboratoryScienceIriOrLabel) -> bool {
		*self == LaboratoryScienceIri || *self == LABORATORY_SCIENCE_LABEL
	}
}
