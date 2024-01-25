/// <https://schema.org/AmusementPark>
pub const AMUSEMENT_PARK_IRI_HTTP: &str = "http://schema.org/AmusementPark";
/// <https://schema.org/AmusementPark>
pub const AMUSEMENT_PARK_IRI_HTTPS: &str = "https://schema.org/AmusementPark";
/// <https://schema.org/AmusementPark>
pub const AMUSEMENT_PARK_LABEL: &str = "AmusementPark";
pub struct AmusementParkIri;
impl PartialEq<&str> for AmusementParkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AMUSEMENT_PARK_IRI_HTTP || *other == AMUSEMENT_PARK_IRI_HTTPS
	}
}
impl PartialEq<AmusementParkIri> for &str {
	fn eq(&self, other: &AmusementParkIri) -> bool {
		*self == AMUSEMENT_PARK_IRI_HTTP || *self == AMUSEMENT_PARK_IRI_HTTPS
	}
}
pub struct AmusementParkIriOrLabel;
impl PartialEq<&str> for AmusementParkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmusementParkIri || *other == AMUSEMENT_PARK_LABEL
	}
}
impl PartialEq<AmusementParkIriOrLabel> for &str {
	fn eq(&self, other: &AmusementParkIriOrLabel) -> bool {
		*self == AmusementParkIri || *self == AMUSEMENT_PARK_LABEL
	}
}
