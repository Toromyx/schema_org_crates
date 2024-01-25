/// <https://schema.org/calories>
pub const CALORIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/calories";
/// <https://schema.org/calories>
pub const CALORIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/calories";
/// <https://schema.org/calories>
pub const CALORIES_PROPERTY_LABEL: &str = "calories";
pub struct CaloriesPropertyIri;
impl PartialEq<&str> for CaloriesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CALORIES_PROPERTY_IRI_HTTP || *other == CALORIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CaloriesPropertyIri> for &str {
	fn eq(&self, other: &CaloriesPropertyIri) -> bool {
		*self == CALORIES_PROPERTY_IRI_HTTP || *self == CALORIES_PROPERTY_IRI_HTTPS
	}
}
pub struct CaloriesPropertyIriOrLabel;
impl PartialEq<&str> for CaloriesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CaloriesPropertyIri || *other == CALORIES_PROPERTY_LABEL
	}
}
impl PartialEq<CaloriesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CaloriesPropertyIriOrLabel) -> bool {
		*self == CaloriesPropertyIri || *self == CALORIES_PROPERTY_LABEL
	}
}
