/// <https://schema.org/QAPage>
pub const QA_PAGE_IRI_HTTP: &str = "http://schema.org/QAPage";
/// <https://schema.org/QAPage>
pub const QA_PAGE_IRI_HTTPS: &str = "https://schema.org/QAPage";
/// <https://schema.org/QAPage>
pub const QA_PAGE_LABEL: &str = "QAPage";
pub struct QaPageIri;
impl PartialEq<&str> for QaPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QA_PAGE_IRI_HTTP || *other == QA_PAGE_IRI_HTTPS
	}
}
impl PartialEq<QaPageIri> for &str {
	fn eq(&self, other: &QaPageIri) -> bool {
		*self == QA_PAGE_IRI_HTTP || *self == QA_PAGE_IRI_HTTPS
	}
}
pub struct QaPageIriOrLabel;
impl PartialEq<&str> for QaPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QaPageIri || *other == QA_PAGE_LABEL
	}
}
impl PartialEq<QaPageIriOrLabel> for &str {
	fn eq(&self, other: &QaPageIriOrLabel) -> bool {
		*self == QaPageIri || *self == QA_PAGE_LABEL
	}
}
