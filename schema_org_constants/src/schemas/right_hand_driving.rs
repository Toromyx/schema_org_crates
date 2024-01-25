/// <https://schema.org/RightHandDriving>
pub const RIGHT_HAND_DRIVING_IRI_HTTP: &str = "http://schema.org/RightHandDriving";
/// <https://schema.org/RightHandDriving>
pub const RIGHT_HAND_DRIVING_IRI_HTTPS: &str = "https://schema.org/RightHandDriving";
/// <https://schema.org/RightHandDriving>
pub const RIGHT_HAND_DRIVING_LABEL: &str = "RightHandDriving";
pub struct RightHandDrivingIri;
impl PartialEq<&str> for RightHandDrivingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RIGHT_HAND_DRIVING_IRI_HTTP || *other == RIGHT_HAND_DRIVING_IRI_HTTPS
	}
}
impl PartialEq<RightHandDrivingIri> for &str {
	fn eq(&self, other: &RightHandDrivingIri) -> bool {
		*self == RIGHT_HAND_DRIVING_IRI_HTTP || *self == RIGHT_HAND_DRIVING_IRI_HTTPS
	}
}
pub struct RightHandDrivingIriOrLabel;
impl PartialEq<&str> for RightHandDrivingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RightHandDrivingIri || *other == RIGHT_HAND_DRIVING_LABEL
	}
}
impl PartialEq<RightHandDrivingIriOrLabel> for &str {
	fn eq(&self, other: &RightHandDrivingIriOrLabel) -> bool {
		*self == RightHandDrivingIri || *self == RIGHT_HAND_DRIVING_LABEL
	}
}
