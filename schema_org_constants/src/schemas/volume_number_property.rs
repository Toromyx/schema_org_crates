/// <https://schema.org/volumeNumber>
pub const VOLUME_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/volumeNumber";
/// <https://schema.org/volumeNumber>
pub const VOLUME_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/volumeNumber";
/// <https://schema.org/volumeNumber>
pub const VOLUME_NUMBER_PROPERTY_LABEL: &str = "volumeNumber";
pub struct VolumeNumberPropertyIri;
impl PartialEq<&str> for VolumeNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VOLUME_NUMBER_PROPERTY_IRI_HTTP || *other == VOLUME_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VolumeNumberPropertyIri> for &str {
	fn eq(&self, other: &VolumeNumberPropertyIri) -> bool {
		*self == VOLUME_NUMBER_PROPERTY_IRI_HTTP || *self == VOLUME_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct VolumeNumberPropertyIriOrLabel;
impl PartialEq<&str> for VolumeNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VolumeNumberPropertyIri || *other == VOLUME_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<VolumeNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VolumeNumberPropertyIriOrLabel) -> bool {
		*self == VolumeNumberPropertyIri || *self == VOLUME_NUMBER_PROPERTY_LABEL
	}
}
