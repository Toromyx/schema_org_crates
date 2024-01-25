/// <https://schema.org/CategoryCodeSet>
pub const CATEGORY_CODE_SET_IRI_HTTP: &str = "http://schema.org/CategoryCodeSet";
/// <https://schema.org/CategoryCodeSet>
pub const CATEGORY_CODE_SET_IRI_HTTPS: &str = "https://schema.org/CategoryCodeSet";
/// <https://schema.org/CategoryCodeSet>
pub const CATEGORY_CODE_SET_LABEL: &str = "CategoryCodeSet";
pub struct CategoryCodeSetIri;
impl PartialEq<&str> for CategoryCodeSetIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATEGORY_CODE_SET_IRI_HTTP || *other == CATEGORY_CODE_SET_IRI_HTTPS
	}
}
impl PartialEq<CategoryCodeSetIri> for &str {
	fn eq(&self, other: &CategoryCodeSetIri) -> bool {
		*self == CATEGORY_CODE_SET_IRI_HTTP || *self == CATEGORY_CODE_SET_IRI_HTTPS
	}
}
pub struct CategoryCodeSetIriOrLabel;
impl PartialEq<&str> for CategoryCodeSetIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CategoryCodeSetIri || *other == CATEGORY_CODE_SET_LABEL
	}
}
impl PartialEq<CategoryCodeSetIriOrLabel> for &str {
	fn eq(&self, other: &CategoryCodeSetIriOrLabel) -> bool {
		*self == CategoryCodeSetIri || *self == CATEGORY_CODE_SET_LABEL
	}
}
