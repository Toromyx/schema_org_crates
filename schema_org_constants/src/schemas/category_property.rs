/// <https://schema.org/category>
pub const CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/category";
/// <https://schema.org/category>
pub const CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/category";
/// <https://schema.org/category>
pub const CATEGORY_PROPERTY_LABEL: &str = "category";
pub struct CategoryPropertyIri;
impl PartialEq<&str> for CategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATEGORY_PROPERTY_IRI_HTTP || *other == CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CategoryPropertyIri> for &str {
	fn eq(&self, other: &CategoryPropertyIri) -> bool {
		*self == CATEGORY_PROPERTY_IRI_HTTP || *self == CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct CategoryPropertyIriOrLabel;
impl PartialEq<&str> for CategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CategoryPropertyIri || *other == CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<CategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CategoryPropertyIriOrLabel) -> bool {
		*self == CategoryPropertyIri || *self == CATEGORY_PROPERTY_LABEL
	}
}
