/// <https://schema.org/cholesterolContent>
pub const CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/cholesterolContent";
/// <https://schema.org/cholesterolContent>
pub const CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cholesterolContent";
/// <https://schema.org/cholesterolContent>
pub const CHOLESTEROL_CONTENT_PROPERTY_LABEL: &str = "cholesterolContent";
pub struct CholesterolContentPropertyIri;
impl PartialEq<&str> for CholesterolContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTP
			|| *other == CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CholesterolContentPropertyIri> for &str {
	fn eq(&self, other: &CholesterolContentPropertyIri) -> bool {
		*self == CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTP
			|| *self == CHOLESTEROL_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct CholesterolContentPropertyIriOrLabel;
impl PartialEq<&str> for CholesterolContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CholesterolContentPropertyIri || *other == CHOLESTEROL_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<CholesterolContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CholesterolContentPropertyIriOrLabel) -> bool {
		*self == CholesterolContentPropertyIri || *self == CHOLESTEROL_CONTENT_PROPERTY_LABEL
	}
}
