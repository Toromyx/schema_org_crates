/// <https://schema.org/openingHours>
pub const OPENING_HOURS_PROPERTY_IRI_HTTP: &str = "http://schema.org/openingHours";
/// <https://schema.org/openingHours>
pub const OPENING_HOURS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/openingHours";
/// <https://schema.org/openingHours>
pub const OPENING_HOURS_PROPERTY_LABEL: &str = "openingHours";
pub struct OpeningHoursPropertyIri;
impl PartialEq<&str> for OpeningHoursPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPENING_HOURS_PROPERTY_IRI_HTTP || *other == OPENING_HOURS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OpeningHoursPropertyIri> for &str {
	fn eq(&self, other: &OpeningHoursPropertyIri) -> bool {
		*self == OPENING_HOURS_PROPERTY_IRI_HTTP || *self == OPENING_HOURS_PROPERTY_IRI_HTTPS
	}
}
pub struct OpeningHoursPropertyIriOrLabel;
impl PartialEq<&str> for OpeningHoursPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpeningHoursPropertyIri || *other == OPENING_HOURS_PROPERTY_LABEL
	}
}
impl PartialEq<OpeningHoursPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OpeningHoursPropertyIriOrLabel) -> bool {
		*self == OpeningHoursPropertyIri || *self == OPENING_HOURS_PROPERTY_LABEL
	}
}
