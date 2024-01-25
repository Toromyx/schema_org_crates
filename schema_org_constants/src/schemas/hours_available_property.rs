/// <https://schema.org/hoursAvailable>
pub const HOURS_AVAILABLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/hoursAvailable";
/// <https://schema.org/hoursAvailable>
pub const HOURS_AVAILABLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hoursAvailable";
/// <https://schema.org/hoursAvailable>
pub const HOURS_AVAILABLE_PROPERTY_LABEL: &str = "hoursAvailable";
pub struct HoursAvailablePropertyIri;
impl PartialEq<&str> for HoursAvailablePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOURS_AVAILABLE_PROPERTY_IRI_HTTP || *other == HOURS_AVAILABLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HoursAvailablePropertyIri> for &str {
	fn eq(&self, other: &HoursAvailablePropertyIri) -> bool {
		*self == HOURS_AVAILABLE_PROPERTY_IRI_HTTP || *self == HOURS_AVAILABLE_PROPERTY_IRI_HTTPS
	}
}
pub struct HoursAvailablePropertyIriOrLabel;
impl PartialEq<&str> for HoursAvailablePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HoursAvailablePropertyIri || *other == HOURS_AVAILABLE_PROPERTY_LABEL
	}
}
impl PartialEq<HoursAvailablePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HoursAvailablePropertyIriOrLabel) -> bool {
		*self == HoursAvailablePropertyIri || *self == HOURS_AVAILABLE_PROPERTY_LABEL
	}
}
