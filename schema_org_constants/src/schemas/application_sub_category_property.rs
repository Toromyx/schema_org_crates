/// <https://schema.org/applicationSubCategory>
pub const APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/applicationSubCategory";
/// <https://schema.org/applicationSubCategory>
pub const APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/applicationSubCategory";
/// <https://schema.org/applicationSubCategory>
pub const APPLICATION_SUB_CATEGORY_PROPERTY_LABEL: &str = "applicationSubCategory";
pub struct ApplicationSubCategoryPropertyIri;
impl PartialEq<&str> for ApplicationSubCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationSubCategoryPropertyIri> for &str {
	fn eq(&self, other: &ApplicationSubCategoryPropertyIri) -> bool {
		*self == APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_SUB_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationSubCategoryPropertyIriOrLabel;
impl PartialEq<&str> for ApplicationSubCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationSubCategoryPropertyIri
			|| *other == APPLICATION_SUB_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationSubCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationSubCategoryPropertyIriOrLabel) -> bool {
		*self == ApplicationSubCategoryPropertyIri
			|| *self == APPLICATION_SUB_CATEGORY_PROPERTY_LABEL
	}
}
