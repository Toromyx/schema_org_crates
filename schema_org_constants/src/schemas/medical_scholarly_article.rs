/// <https://schema.org/MedicalScholarlyArticle>
pub const MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTP: &str = "http://schema.org/MedicalScholarlyArticle";
/// <https://schema.org/MedicalScholarlyArticle>
pub const MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTPS: &str = "https://schema.org/MedicalScholarlyArticle";
/// <https://schema.org/MedicalScholarlyArticle>
pub const MEDICAL_SCHOLARLY_ARTICLE_LABEL: &str = "MedicalScholarlyArticle";
pub struct MedicalScholarlyArticleIri;
impl PartialEq<&str> for MedicalScholarlyArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTP
			|| *other == MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<MedicalScholarlyArticleIri> for &str {
	fn eq(&self, other: &MedicalScholarlyArticleIri) -> bool {
		*self == MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTP || *self == MEDICAL_SCHOLARLY_ARTICLE_IRI_HTTPS
	}
}
pub struct MedicalScholarlyArticleIriOrLabel;
impl PartialEq<&str> for MedicalScholarlyArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalScholarlyArticleIri || *other == MEDICAL_SCHOLARLY_ARTICLE_LABEL
	}
}
impl PartialEq<MedicalScholarlyArticleIriOrLabel> for &str {
	fn eq(&self, other: &MedicalScholarlyArticleIriOrLabel) -> bool {
		*self == MedicalScholarlyArticleIri || *self == MEDICAL_SCHOLARLY_ARTICLE_LABEL
	}
}
