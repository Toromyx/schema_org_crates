/// <https://schema.org/TaxiService>
pub const TAXI_SERVICE_IRI_HTTP: &str = "http://schema.org/TaxiService";
/// <https://schema.org/TaxiService>
pub const TAXI_SERVICE_IRI_HTTPS: &str = "https://schema.org/TaxiService";
/// <https://schema.org/TaxiService>
pub const TAXI_SERVICE_LABEL: &str = "TaxiService";
pub struct TaxiServiceIri;
impl PartialEq<&str> for TaxiServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXI_SERVICE_IRI_HTTP || *other == TAXI_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<TaxiServiceIri> for &str {
	fn eq(&self, other: &TaxiServiceIri) -> bool {
		*self == TAXI_SERVICE_IRI_HTTP || *self == TAXI_SERVICE_IRI_HTTPS
	}
}
pub struct TaxiServiceIriOrLabel;
impl PartialEq<&str> for TaxiServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxiServiceIri || *other == TAXI_SERVICE_LABEL
	}
}
impl PartialEq<TaxiServiceIriOrLabel> for &str {
	fn eq(&self, other: &TaxiServiceIriOrLabel) -> bool {
		*self == TaxiServiceIri || *self == TAXI_SERVICE_LABEL
	}
}
