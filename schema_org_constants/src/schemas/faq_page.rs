/// <https://schema.org/FAQPage>
pub const FAQ_PAGE_IRI_HTTP: &str = "http://schema.org/FAQPage";
/// <https://schema.org/FAQPage>
pub const FAQ_PAGE_IRI_HTTPS: &str = "https://schema.org/FAQPage";
/// <https://schema.org/FAQPage>
pub const FAQ_PAGE_LABEL: &str = "FAQPage";
pub struct FaqPageIri;
impl PartialEq<&str> for FaqPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAQ_PAGE_IRI_HTTP || *other == FAQ_PAGE_IRI_HTTPS
	}
}
impl PartialEq<FaqPageIri> for &str {
	fn eq(&self, other: &FaqPageIri) -> bool {
		*self == FAQ_PAGE_IRI_HTTP || *self == FAQ_PAGE_IRI_HTTPS
	}
}
pub struct FaqPageIriOrLabel;
impl PartialEq<&str> for FaqPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FaqPageIri || *other == FAQ_PAGE_LABEL
	}
}
impl PartialEq<FaqPageIriOrLabel> for &str {
	fn eq(&self, other: &FaqPageIriOrLabel) -> bool {
		*self == FaqPageIri || *self == FAQ_PAGE_LABEL
	}
}
