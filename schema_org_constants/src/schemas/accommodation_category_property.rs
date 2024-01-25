/// <https://schema.org/accommodationCategory>
pub const ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/accommodationCategory";
/// <https://schema.org/accommodationCategory>
pub const ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accommodationCategory";
/// <https://schema.org/accommodationCategory>
pub const ACCOMMODATION_CATEGORY_PROPERTY_LABEL: &str = "accommodationCategory";
pub struct AccommodationCategoryPropertyIri;
impl PartialEq<&str> for AccommodationCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccommodationCategoryPropertyIri> for &str {
	fn eq(&self, other: &AccommodationCategoryPropertyIri) -> bool {
		*self == ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == ACCOMMODATION_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct AccommodationCategoryPropertyIriOrLabel;
impl PartialEq<&str> for AccommodationCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccommodationCategoryPropertyIri
			|| *other == ACCOMMODATION_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<AccommodationCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccommodationCategoryPropertyIriOrLabel) -> bool {
		*self == AccommodationCategoryPropertyIri || *self == ACCOMMODATION_CATEGORY_PROPERTY_LABEL
	}
}
