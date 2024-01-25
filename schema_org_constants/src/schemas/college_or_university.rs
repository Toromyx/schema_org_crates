/// <https://schema.org/CollegeOrUniversity>
pub const COLLEGE_OR_UNIVERSITY_IRI_HTTP: &str = "http://schema.org/CollegeOrUniversity";
/// <https://schema.org/CollegeOrUniversity>
pub const COLLEGE_OR_UNIVERSITY_IRI_HTTPS: &str = "https://schema.org/CollegeOrUniversity";
/// <https://schema.org/CollegeOrUniversity>
pub const COLLEGE_OR_UNIVERSITY_LABEL: &str = "CollegeOrUniversity";
pub struct CollegeOrUniversityIri;
impl PartialEq<&str> for CollegeOrUniversityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLEGE_OR_UNIVERSITY_IRI_HTTP || *other == COLLEGE_OR_UNIVERSITY_IRI_HTTPS
	}
}
impl PartialEq<CollegeOrUniversityIri> for &str {
	fn eq(&self, other: &CollegeOrUniversityIri) -> bool {
		*self == COLLEGE_OR_UNIVERSITY_IRI_HTTP || *self == COLLEGE_OR_UNIVERSITY_IRI_HTTPS
	}
}
pub struct CollegeOrUniversityIriOrLabel;
impl PartialEq<&str> for CollegeOrUniversityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CollegeOrUniversityIri || *other == COLLEGE_OR_UNIVERSITY_LABEL
	}
}
impl PartialEq<CollegeOrUniversityIriOrLabel> for &str {
	fn eq(&self, other: &CollegeOrUniversityIriOrLabel) -> bool {
		*self == CollegeOrUniversityIri || *self == COLLEGE_OR_UNIVERSITY_LABEL
	}
}
