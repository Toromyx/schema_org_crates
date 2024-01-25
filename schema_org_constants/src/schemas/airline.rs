/// <https://schema.org/Airline>
pub const AIRLINE_IRI_HTTP: &str = "http://schema.org/Airline";
/// <https://schema.org/Airline>
pub const AIRLINE_IRI_HTTPS: &str = "https://schema.org/Airline";
/// <https://schema.org/Airline>
pub const AIRLINE_LABEL: &str = "Airline";
pub struct AirlineIri;
impl PartialEq<&str> for AirlineIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AIRLINE_IRI_HTTP || *other == AIRLINE_IRI_HTTPS
	}
}
impl PartialEq<AirlineIri> for &str {
	fn eq(&self, other: &AirlineIri) -> bool {
		*self == AIRLINE_IRI_HTTP || *self == AIRLINE_IRI_HTTPS
	}
}
pub struct AirlineIriOrLabel;
impl PartialEq<&str> for AirlineIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AirlineIri || *other == AIRLINE_LABEL
	}
}
impl PartialEq<AirlineIriOrLabel> for &str {
	fn eq(&self, other: &AirlineIriOrLabel) -> bool {
		*self == AirlineIri || *self == AIRLINE_LABEL
	}
}
