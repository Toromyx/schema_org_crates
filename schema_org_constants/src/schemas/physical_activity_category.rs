/// <https://schema.org/PhysicalActivityCategory>
pub const PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTP: &str = "http://schema.org/PhysicalActivityCategory";
/// <https://schema.org/PhysicalActivityCategory>
pub const PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTPS: &str =
	"https://schema.org/PhysicalActivityCategory";
/// <https://schema.org/PhysicalActivityCategory>
pub const PHYSICAL_ACTIVITY_CATEGORY_LABEL: &str = "PhysicalActivityCategory";
pub struct PhysicalActivityCategoryIri;
impl PartialEq<&str> for PhysicalActivityCategoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTP
			|| *other == PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTPS
	}
}
impl PartialEq<PhysicalActivityCategoryIri> for &str {
	fn eq(&self, other: &PhysicalActivityCategoryIri) -> bool {
		*self == PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTP
			|| *self == PHYSICAL_ACTIVITY_CATEGORY_IRI_HTTPS
	}
}
pub struct PhysicalActivityCategoryIriOrLabel;
impl PartialEq<&str> for PhysicalActivityCategoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicalActivityCategoryIri || *other == PHYSICAL_ACTIVITY_CATEGORY_LABEL
	}
}
impl PartialEq<PhysicalActivityCategoryIriOrLabel> for &str {
	fn eq(&self, other: &PhysicalActivityCategoryIriOrLabel) -> bool {
		*self == PhysicalActivityCategoryIri || *self == PHYSICAL_ACTIVITY_CATEGORY_LABEL
	}
}
