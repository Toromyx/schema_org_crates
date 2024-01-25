/// <https://schema.org/hasCategoryCode>
pub const HAS_CATEGORY_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasCategoryCode";
/// <https://schema.org/hasCategoryCode>
pub const HAS_CATEGORY_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasCategoryCode";
/// <https://schema.org/hasCategoryCode>
pub const HAS_CATEGORY_CODE_PROPERTY_LABEL: &str = "hasCategoryCode";
pub struct HasCategoryCodePropertyIri;
impl PartialEq<&str> for HasCategoryCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_CATEGORY_CODE_PROPERTY_IRI_HTTP
			|| *other == HAS_CATEGORY_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasCategoryCodePropertyIri> for &str {
	fn eq(&self, other: &HasCategoryCodePropertyIri) -> bool {
		*self == HAS_CATEGORY_CODE_PROPERTY_IRI_HTTP
			|| *self == HAS_CATEGORY_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct HasCategoryCodePropertyIriOrLabel;
impl PartialEq<&str> for HasCategoryCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasCategoryCodePropertyIri || *other == HAS_CATEGORY_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<HasCategoryCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasCategoryCodePropertyIriOrLabel) -> bool {
		*self == HasCategoryCodePropertyIri || *self == HAS_CATEGORY_CODE_PROPERTY_LABEL
	}
}
