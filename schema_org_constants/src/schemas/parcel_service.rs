/// <https://schema.org/ParcelService>
pub const PARCEL_SERVICE_IRI_HTTP: &str = "http://schema.org/ParcelService";
/// <https://schema.org/ParcelService>
pub const PARCEL_SERVICE_IRI_HTTPS: &str = "https://schema.org/ParcelService";
/// <https://schema.org/ParcelService>
pub const PARCEL_SERVICE_LABEL: &str = "ParcelService";
pub struct ParcelServiceIri;
impl PartialEq<&str> for ParcelServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARCEL_SERVICE_IRI_HTTP || *other == PARCEL_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<ParcelServiceIri> for &str {
	fn eq(&self, other: &ParcelServiceIri) -> bool {
		*self == PARCEL_SERVICE_IRI_HTTP || *self == PARCEL_SERVICE_IRI_HTTPS
	}
}
pub struct ParcelServiceIriOrLabel;
impl PartialEq<&str> for ParcelServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParcelServiceIri || *other == PARCEL_SERVICE_LABEL
	}
}
impl PartialEq<ParcelServiceIriOrLabel> for &str {
	fn eq(&self, other: &ParcelServiceIriOrLabel) -> bool {
		*self == ParcelServiceIri || *self == PARCEL_SERVICE_LABEL
	}
}
