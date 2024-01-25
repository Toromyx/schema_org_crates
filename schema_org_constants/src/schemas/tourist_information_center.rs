/// <https://schema.org/TouristInformationCenter>
pub const TOURIST_INFORMATION_CENTER_IRI_HTTP: &str = "http://schema.org/TouristInformationCenter";
/// <https://schema.org/TouristInformationCenter>
pub const TOURIST_INFORMATION_CENTER_IRI_HTTPS: &str =
	"https://schema.org/TouristInformationCenter";
/// <https://schema.org/TouristInformationCenter>
pub const TOURIST_INFORMATION_CENTER_LABEL: &str = "TouristInformationCenter";
pub struct TouristInformationCenterIri;
impl PartialEq<&str> for TouristInformationCenterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOURIST_INFORMATION_CENTER_IRI_HTTP
			|| *other == TOURIST_INFORMATION_CENTER_IRI_HTTPS
	}
}
impl PartialEq<TouristInformationCenterIri> for &str {
	fn eq(&self, other: &TouristInformationCenterIri) -> bool {
		*self == TOURIST_INFORMATION_CENTER_IRI_HTTP
			|| *self == TOURIST_INFORMATION_CENTER_IRI_HTTPS
	}
}
pub struct TouristInformationCenterIriOrLabel;
impl PartialEq<&str> for TouristInformationCenterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TouristInformationCenterIri || *other == TOURIST_INFORMATION_CENTER_LABEL
	}
}
impl PartialEq<TouristInformationCenterIriOrLabel> for &str {
	fn eq(&self, other: &TouristInformationCenterIriOrLabel) -> bool {
		*self == TouristInformationCenterIri || *self == TOURIST_INFORMATION_CENTER_LABEL
	}
}
