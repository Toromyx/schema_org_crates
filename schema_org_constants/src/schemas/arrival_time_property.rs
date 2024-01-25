/// <https://schema.org/arrivalTime>
pub const ARRIVAL_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalTime";
/// <https://schema.org/arrivalTime>
pub const ARRIVAL_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalTime";
/// <https://schema.org/arrivalTime>
pub const ARRIVAL_TIME_PROPERTY_LABEL: &str = "arrivalTime";
pub struct ArrivalTimePropertyIri;
impl PartialEq<&str> for ArrivalTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_TIME_PROPERTY_IRI_HTTP || *other == ARRIVAL_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalTimePropertyIri> for &str {
	fn eq(&self, other: &ArrivalTimePropertyIri) -> bool {
		*self == ARRIVAL_TIME_PROPERTY_IRI_HTTP || *self == ARRIVAL_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalTimePropertyIriOrLabel;
impl PartialEq<&str> for ArrivalTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalTimePropertyIri || *other == ARRIVAL_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalTimePropertyIriOrLabel) -> bool {
		*self == ArrivalTimePropertyIri || *self == ARRIVAL_TIME_PROPERTY_LABEL
	}
}
