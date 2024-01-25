/// <https://schema.org/ElementarySchool>
pub const ELEMENTARY_SCHOOL_IRI_HTTP: &str = "http://schema.org/ElementarySchool";
/// <https://schema.org/ElementarySchool>
pub const ELEMENTARY_SCHOOL_IRI_HTTPS: &str = "https://schema.org/ElementarySchool";
/// <https://schema.org/ElementarySchool>
pub const ELEMENTARY_SCHOOL_LABEL: &str = "ElementarySchool";
pub struct ElementarySchoolIri;
impl PartialEq<&str> for ElementarySchoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELEMENTARY_SCHOOL_IRI_HTTP || *other == ELEMENTARY_SCHOOL_IRI_HTTPS
	}
}
impl PartialEq<ElementarySchoolIri> for &str {
	fn eq(&self, other: &ElementarySchoolIri) -> bool {
		*self == ELEMENTARY_SCHOOL_IRI_HTTP || *self == ELEMENTARY_SCHOOL_IRI_HTTPS
	}
}
pub struct ElementarySchoolIriOrLabel;
impl PartialEq<&str> for ElementarySchoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ElementarySchoolIri || *other == ELEMENTARY_SCHOOL_LABEL
	}
}
impl PartialEq<ElementarySchoolIriOrLabel> for &str {
	fn eq(&self, other: &ElementarySchoolIriOrLabel) -> bool {
		*self == ElementarySchoolIri || *self == ELEMENTARY_SCHOOL_LABEL
	}
}
