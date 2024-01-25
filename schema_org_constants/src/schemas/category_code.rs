/// <https://schema.org/CategoryCode>
pub const CATEGORY_CODE_IRI_HTTP: &str = "http://schema.org/CategoryCode";
/// <https://schema.org/CategoryCode>
pub const CATEGORY_CODE_IRI_HTTPS: &str = "https://schema.org/CategoryCode";
/// <https://schema.org/CategoryCode>
pub const CATEGORY_CODE_LABEL: &str = "CategoryCode";
pub struct CategoryCodeIri;
impl PartialEq<&str> for CategoryCodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATEGORY_CODE_IRI_HTTP || *other == CATEGORY_CODE_IRI_HTTPS
	}
}
impl PartialEq<CategoryCodeIri> for &str {
	fn eq(&self, other: &CategoryCodeIri) -> bool {
		*self == CATEGORY_CODE_IRI_HTTP || *self == CATEGORY_CODE_IRI_HTTPS
	}
}
pub struct CategoryCodeIriOrLabel;
impl PartialEq<&str> for CategoryCodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CategoryCodeIri || *other == CATEGORY_CODE_LABEL
	}
}
impl PartialEq<CategoryCodeIriOrLabel> for &str {
	fn eq(&self, other: &CategoryCodeIriOrLabel) -> bool {
		*self == CategoryCodeIri || *self == CATEGORY_CODE_LABEL
	}
}
