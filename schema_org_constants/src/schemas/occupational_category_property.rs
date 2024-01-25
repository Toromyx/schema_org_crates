/// <https://schema.org/occupationalCategory>
pub const OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/occupationalCategory";
/// <https://schema.org/occupationalCategory>
pub const OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/occupationalCategory";
/// <https://schema.org/occupationalCategory>
pub const OCCUPATIONAL_CATEGORY_PROPERTY_LABEL: &str = "occupationalCategory";
pub struct OccupationalCategoryPropertyIri;
impl PartialEq<&str> for OccupationalCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OccupationalCategoryPropertyIri> for &str {
	fn eq(&self, other: &OccupationalCategoryPropertyIri) -> bool {
		*self == OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == OCCUPATIONAL_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct OccupationalCategoryPropertyIriOrLabel;
impl PartialEq<&str> for OccupationalCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationalCategoryPropertyIri || *other == OCCUPATIONAL_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<OccupationalCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OccupationalCategoryPropertyIriOrLabel) -> bool {
		*self == OccupationalCategoryPropertyIri || *self == OCCUPATIONAL_CATEGORY_PROPERTY_LABEL
	}
}
