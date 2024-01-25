/// <https://schema.org/seatingCapacity>
pub const SEATING_CAPACITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/seatingCapacity";
/// <https://schema.org/seatingCapacity>
pub const SEATING_CAPACITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seatingCapacity";
/// <https://schema.org/seatingCapacity>
pub const SEATING_CAPACITY_PROPERTY_LABEL: &str = "seatingCapacity";
pub struct SeatingCapacityPropertyIri;
impl PartialEq<&str> for SeatingCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEATING_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == SEATING_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeatingCapacityPropertyIri> for &str {
	fn eq(&self, other: &SeatingCapacityPropertyIri) -> bool {
		*self == SEATING_CAPACITY_PROPERTY_IRI_HTTP || *self == SEATING_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct SeatingCapacityPropertyIriOrLabel;
impl PartialEq<&str> for SeatingCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatingCapacityPropertyIri || *other == SEATING_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<SeatingCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeatingCapacityPropertyIriOrLabel) -> bool {
		*self == SeatingCapacityPropertyIri || *self == SEATING_CAPACITY_PROPERTY_LABEL
	}
}
