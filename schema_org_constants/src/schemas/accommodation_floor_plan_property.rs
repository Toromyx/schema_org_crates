/// <https://schema.org/accommodationFloorPlan>
pub const ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/accommodationFloorPlan";
/// <https://schema.org/accommodationFloorPlan>
pub const ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accommodationFloorPlan";
/// <https://schema.org/accommodationFloorPlan>
pub const ACCOMMODATION_FLOOR_PLAN_PROPERTY_LABEL: &str = "accommodationFloorPlan";
pub struct AccommodationFloorPlanPropertyIri;
impl PartialEq<&str> for AccommodationFloorPlanPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTP
			|| *other == ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccommodationFloorPlanPropertyIri> for &str {
	fn eq(&self, other: &AccommodationFloorPlanPropertyIri) -> bool {
		*self == ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTP
			|| *self == ACCOMMODATION_FLOOR_PLAN_PROPERTY_IRI_HTTPS
	}
}
pub struct AccommodationFloorPlanPropertyIriOrLabel;
impl PartialEq<&str> for AccommodationFloorPlanPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccommodationFloorPlanPropertyIri
			|| *other == ACCOMMODATION_FLOOR_PLAN_PROPERTY_LABEL
	}
}
impl PartialEq<AccommodationFloorPlanPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccommodationFloorPlanPropertyIriOrLabel) -> bool {
		*self == AccommodationFloorPlanPropertyIri
			|| *self == ACCOMMODATION_FLOOR_PLAN_PROPERTY_LABEL
	}
}
