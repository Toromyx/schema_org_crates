/// <https://schema.org/fromLocation>
pub const FROM_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/fromLocation";
/// <https://schema.org/fromLocation>
pub const FROM_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fromLocation";
/// <https://schema.org/fromLocation>
pub const FROM_LOCATION_PROPERTY_LABEL: &str = "fromLocation";
pub struct FromLocationPropertyIri;
impl PartialEq<&str> for FromLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FROM_LOCATION_PROPERTY_IRI_HTTP || *other == FROM_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FromLocationPropertyIri> for &str {
	fn eq(&self, other: &FromLocationPropertyIri) -> bool {
		*self == FROM_LOCATION_PROPERTY_IRI_HTTP || *self == FROM_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct FromLocationPropertyIriOrLabel;
impl PartialEq<&str> for FromLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FromLocationPropertyIri || *other == FROM_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<FromLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FromLocationPropertyIriOrLabel) -> bool {
		*self == FromLocationPropertyIri || *self == FROM_LOCATION_PROPERTY_LABEL
	}
}
