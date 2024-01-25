/// <https://schema.org/expectedArrivalFrom>
pub const EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/expectedArrivalFrom";
/// <https://schema.org/expectedArrivalFrom>
pub const EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/expectedArrivalFrom";
/// <https://schema.org/expectedArrivalFrom>
pub const EXPECTED_ARRIVAL_FROM_PROPERTY_LABEL: &str = "expectedArrivalFrom";
pub struct ExpectedArrivalFromPropertyIri;
impl PartialEq<&str> for ExpectedArrivalFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTP
			|| *other == EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpectedArrivalFromPropertyIri> for &str {
	fn eq(&self, other: &ExpectedArrivalFromPropertyIri) -> bool {
		*self == EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTP
			|| *self == EXPECTED_ARRIVAL_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpectedArrivalFromPropertyIriOrLabel;
impl PartialEq<&str> for ExpectedArrivalFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpectedArrivalFromPropertyIri || *other == EXPECTED_ARRIVAL_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<ExpectedArrivalFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpectedArrivalFromPropertyIriOrLabel) -> bool {
		*self == ExpectedArrivalFromPropertyIri || *self == EXPECTED_ARRIVAL_FROM_PROPERTY_LABEL
	}
}
