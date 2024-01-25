/// <https://schema.org/Park>
pub const PARK_IRI_HTTP: &str = "http://schema.org/Park";
/// <https://schema.org/Park>
pub const PARK_IRI_HTTPS: &str = "https://schema.org/Park";
/// <https://schema.org/Park>
pub const PARK_LABEL: &str = "Park";
pub struct ParkIri;
impl PartialEq<&str> for ParkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARK_IRI_HTTP || *other == PARK_IRI_HTTPS
	}
}
impl PartialEq<ParkIri> for &str {
	fn eq(&self, other: &ParkIri) -> bool {
		*self == PARK_IRI_HTTP || *self == PARK_IRI_HTTPS
	}
}
pub struct ParkIriOrLabel;
impl PartialEq<&str> for ParkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParkIri || *other == PARK_LABEL
	}
}
impl PartialEq<ParkIriOrLabel> for &str {
	fn eq(&self, other: &ParkIriOrLabel) -> bool {
		*self == ParkIri || *self == PARK_LABEL
	}
}
