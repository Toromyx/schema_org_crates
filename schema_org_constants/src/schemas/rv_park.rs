/// <https://schema.org/RVPark>
pub const RV_PARK_IRI_HTTP: &str = "http://schema.org/RVPark";
/// <https://schema.org/RVPark>
pub const RV_PARK_IRI_HTTPS: &str = "https://schema.org/RVPark";
/// <https://schema.org/RVPark>
pub const RV_PARK_LABEL: &str = "RVPark";
pub struct RvParkIri;
impl PartialEq<&str> for RvParkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RV_PARK_IRI_HTTP || *other == RV_PARK_IRI_HTTPS
	}
}
impl PartialEq<RvParkIri> for &str {
	fn eq(&self, other: &RvParkIri) -> bool {
		*self == RV_PARK_IRI_HTTP || *self == RV_PARK_IRI_HTTPS
	}
}
pub struct RvParkIriOrLabel;
impl PartialEq<&str> for RvParkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RvParkIri || *other == RV_PARK_LABEL
	}
}
impl PartialEq<RvParkIriOrLabel> for &str {
	fn eq(&self, other: &RvParkIriOrLabel) -> bool {
		*self == RvParkIri || *self == RV_PARK_LABEL
	}
}
