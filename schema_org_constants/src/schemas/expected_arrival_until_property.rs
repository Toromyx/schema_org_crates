/// <https://schema.org/expectedArrivalUntil>
pub const EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTP: &str = "http://schema.org/expectedArrivalUntil";
/// <https://schema.org/expectedArrivalUntil>
pub const EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/expectedArrivalUntil";
/// <https://schema.org/expectedArrivalUntil>
pub const EXPECTED_ARRIVAL_UNTIL_PROPERTY_LABEL: &str = "expectedArrivalUntil";
pub struct ExpectedArrivalUntilPropertyIri;
impl PartialEq<&str> for ExpectedArrivalUntilPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTP
			|| *other == EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExpectedArrivalUntilPropertyIri> for &str {
	fn eq(&self, other: &ExpectedArrivalUntilPropertyIri) -> bool {
		*self == EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTP
			|| *self == EXPECTED_ARRIVAL_UNTIL_PROPERTY_IRI_HTTPS
	}
}
pub struct ExpectedArrivalUntilPropertyIriOrLabel;
impl PartialEq<&str> for ExpectedArrivalUntilPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExpectedArrivalUntilPropertyIri || *other == EXPECTED_ARRIVAL_UNTIL_PROPERTY_LABEL
	}
}
impl PartialEq<ExpectedArrivalUntilPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExpectedArrivalUntilPropertyIriOrLabel) -> bool {
		*self == ExpectedArrivalUntilPropertyIri || *self == EXPECTED_ARRIVAL_UNTIL_PROPERTY_LABEL
	}
}
