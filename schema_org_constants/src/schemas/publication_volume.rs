/// <https://schema.org/PublicationVolume>
pub const PUBLICATION_VOLUME_IRI_HTTP: &str = "http://schema.org/PublicationVolume";
/// <https://schema.org/PublicationVolume>
pub const PUBLICATION_VOLUME_IRI_HTTPS: &str = "https://schema.org/PublicationVolume";
/// <https://schema.org/PublicationVolume>
pub const PUBLICATION_VOLUME_LABEL: &str = "PublicationVolume";
pub struct PublicationVolumeIri;
impl PartialEq<&str> for PublicationVolumeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLICATION_VOLUME_IRI_HTTP || *other == PUBLICATION_VOLUME_IRI_HTTPS
	}
}
impl PartialEq<PublicationVolumeIri> for &str {
	fn eq(&self, other: &PublicationVolumeIri) -> bool {
		*self == PUBLICATION_VOLUME_IRI_HTTP || *self == PUBLICATION_VOLUME_IRI_HTTPS
	}
}
pub struct PublicationVolumeIriOrLabel;
impl PartialEq<&str> for PublicationVolumeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicationVolumeIri || *other == PUBLICATION_VOLUME_LABEL
	}
}
impl PartialEq<PublicationVolumeIriOrLabel> for &str {
	fn eq(&self, other: &PublicationVolumeIriOrLabel) -> bool {
		*self == PublicationVolumeIri || *self == PUBLICATION_VOLUME_LABEL
	}
}
