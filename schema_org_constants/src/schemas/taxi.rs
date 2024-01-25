/// <https://schema.org/Taxi>
#[deprecated = "This schema is superseded by <https://schema.org/TaxiService>."]
pub const TAXI_IRI_HTTP: &str = "http://schema.org/Taxi";
/// <https://schema.org/Taxi>
#[deprecated = "This schema is superseded by <https://schema.org/TaxiService>."]
pub const TAXI_IRI_HTTPS: &str = "https://schema.org/Taxi";
/// <https://schema.org/Taxi>
#[deprecated = "This schema is superseded by <https://schema.org/TaxiService>."]
pub const TAXI_LABEL: &str = "Taxi";
pub struct TaxiIri;
impl PartialEq<&str> for TaxiIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXI_IRI_HTTP || *other == TAXI_IRI_HTTPS
	}
}
impl PartialEq<TaxiIri> for &str {
	fn eq(&self, other: &TaxiIri) -> bool {
		*self == TAXI_IRI_HTTP || *self == TAXI_IRI_HTTPS
	}
}
pub struct TaxiIriOrLabel;
impl PartialEq<&str> for TaxiIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxiIri || *other == TAXI_LABEL
	}
}
impl PartialEq<TaxiIriOrLabel> for &str {
	fn eq(&self, other: &TaxiIriOrLabel) -> bool {
		*self == TaxiIri || *self == TAXI_LABEL
	}
}
