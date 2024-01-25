/// <https://schema.org/applicationCategory>
pub const APPLICATION_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/applicationCategory";
/// <https://schema.org/applicationCategory>
pub const APPLICATION_CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/applicationCategory";
/// <https://schema.org/applicationCategory>
pub const APPLICATION_CATEGORY_PROPERTY_LABEL: &str = "applicationCategory";
pub struct ApplicationCategoryPropertyIri;
impl PartialEq<&str> for ApplicationCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLICATION_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == APPLICATION_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ApplicationCategoryPropertyIri> for &str {
	fn eq(&self, other: &ApplicationCategoryPropertyIri) -> bool {
		*self == APPLICATION_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == APPLICATION_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct ApplicationCategoryPropertyIriOrLabel;
impl PartialEq<&str> for ApplicationCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplicationCategoryPropertyIri || *other == APPLICATION_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<ApplicationCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ApplicationCategoryPropertyIriOrLabel) -> bool {
		*self == ApplicationCategoryPropertyIri || *self == APPLICATION_CATEGORY_PROPERTY_LABEL
	}
}
