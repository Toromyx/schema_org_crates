/// <https://schema.org/seatingType>
pub const SEATING_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/seatingType";
/// <https://schema.org/seatingType>
pub const SEATING_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seatingType";
/// <https://schema.org/seatingType>
pub const SEATING_TYPE_PROPERTY_LABEL: &str = "seatingType";
pub struct SeatingTypePropertyIri;
impl PartialEq<&str> for SeatingTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEATING_TYPE_PROPERTY_IRI_HTTP || *other == SEATING_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeatingTypePropertyIri> for &str {
	fn eq(&self, other: &SeatingTypePropertyIri) -> bool {
		*self == SEATING_TYPE_PROPERTY_IRI_HTTP || *self == SEATING_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct SeatingTypePropertyIriOrLabel;
impl PartialEq<&str> for SeatingTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatingTypePropertyIri || *other == SEATING_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<SeatingTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeatingTypePropertyIriOrLabel) -> bool {
		*self == SeatingTypePropertyIri || *self == SEATING_TYPE_PROPERTY_LABEL
	}
}
