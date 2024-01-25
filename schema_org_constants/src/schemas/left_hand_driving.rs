/// <https://schema.org/LeftHandDriving>
pub const LEFT_HAND_DRIVING_IRI_HTTP: &str = "http://schema.org/LeftHandDriving";
/// <https://schema.org/LeftHandDriving>
pub const LEFT_HAND_DRIVING_IRI_HTTPS: &str = "https://schema.org/LeftHandDriving";
/// <https://schema.org/LeftHandDriving>
pub const LEFT_HAND_DRIVING_LABEL: &str = "LeftHandDriving";
pub struct LeftHandDrivingIri;
impl PartialEq<&str> for LeftHandDrivingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEFT_HAND_DRIVING_IRI_HTTP || *other == LEFT_HAND_DRIVING_IRI_HTTPS
	}
}
impl PartialEq<LeftHandDrivingIri> for &str {
	fn eq(&self, other: &LeftHandDrivingIri) -> bool {
		*self == LEFT_HAND_DRIVING_IRI_HTTP || *self == LEFT_HAND_DRIVING_IRI_HTTPS
	}
}
pub struct LeftHandDrivingIriOrLabel;
impl PartialEq<&str> for LeftHandDrivingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LeftHandDrivingIri || *other == LEFT_HAND_DRIVING_LABEL
	}
}
impl PartialEq<LeftHandDrivingIriOrLabel> for &str {
	fn eq(&self, other: &LeftHandDrivingIriOrLabel) -> bool {
		*self == LeftHandDrivingIri || *self == LEFT_HAND_DRIVING_LABEL
	}
}
