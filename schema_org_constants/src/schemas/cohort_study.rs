/// <https://schema.org/CohortStudy>
pub const COHORT_STUDY_IRI_HTTP: &str = "http://schema.org/CohortStudy";
/// <https://schema.org/CohortStudy>
pub const COHORT_STUDY_IRI_HTTPS: &str = "https://schema.org/CohortStudy";
/// <https://schema.org/CohortStudy>
pub const COHORT_STUDY_LABEL: &str = "CohortStudy";
pub struct CohortStudyIri;
impl PartialEq<&str> for CohortStudyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COHORT_STUDY_IRI_HTTP || *other == COHORT_STUDY_IRI_HTTPS
	}
}
impl PartialEq<CohortStudyIri> for &str {
	fn eq(&self, other: &CohortStudyIri) -> bool {
		*self == COHORT_STUDY_IRI_HTTP || *self == COHORT_STUDY_IRI_HTTPS
	}
}
pub struct CohortStudyIriOrLabel;
impl PartialEq<&str> for CohortStudyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CohortStudyIri || *other == COHORT_STUDY_LABEL
	}
}
impl PartialEq<CohortStudyIriOrLabel> for &str {
	fn eq(&self, other: &CohortStudyIriOrLabel) -> bool {
		*self == CohortStudyIri || *self == COHORT_STUDY_LABEL
	}
}
