/// <https://schema.org/TaxiStand>
pub const TAXI_STAND_IRI_HTTP: &str = "http://schema.org/TaxiStand";
/// <https://schema.org/TaxiStand>
pub const TAXI_STAND_IRI_HTTPS: &str = "https://schema.org/TaxiStand";
/// <https://schema.org/TaxiStand>
pub const TAXI_STAND_LABEL: &str = "TaxiStand";
pub struct TaxiStandIri;
impl PartialEq<&str> for TaxiStandIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXI_STAND_IRI_HTTP || *other == TAXI_STAND_IRI_HTTPS
	}
}
impl PartialEq<TaxiStandIri> for &str {
	fn eq(&self, other: &TaxiStandIri) -> bool {
		*self == TAXI_STAND_IRI_HTTP || *self == TAXI_STAND_IRI_HTTPS
	}
}
pub struct TaxiStandIriOrLabel;
impl PartialEq<&str> for TaxiStandIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxiStandIri || *other == TAXI_STAND_LABEL
	}
}
impl PartialEq<TaxiStandIriOrLabel> for &str {
	fn eq(&self, other: &TaxiStandIriOrLabel) -> bool {
		*self == TaxiStandIri || *self == TAXI_STAND_LABEL
	}
}
