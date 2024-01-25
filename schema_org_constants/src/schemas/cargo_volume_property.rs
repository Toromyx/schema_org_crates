/// <https://schema.org/cargoVolume>
pub const CARGO_VOLUME_PROPERTY_IRI_HTTP: &str = "http://schema.org/cargoVolume";
/// <https://schema.org/cargoVolume>
pub const CARGO_VOLUME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cargoVolume";
/// <https://schema.org/cargoVolume>
pub const CARGO_VOLUME_PROPERTY_LABEL: &str = "cargoVolume";
pub struct CargoVolumePropertyIri;
impl PartialEq<&str> for CargoVolumePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARGO_VOLUME_PROPERTY_IRI_HTTP || *other == CARGO_VOLUME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CargoVolumePropertyIri> for &str {
	fn eq(&self, other: &CargoVolumePropertyIri) -> bool {
		*self == CARGO_VOLUME_PROPERTY_IRI_HTTP || *self == CARGO_VOLUME_PROPERTY_IRI_HTTPS
	}
}
pub struct CargoVolumePropertyIriOrLabel;
impl PartialEq<&str> for CargoVolumePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CargoVolumePropertyIri || *other == CARGO_VOLUME_PROPERTY_LABEL
	}
}
impl PartialEq<CargoVolumePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CargoVolumePropertyIriOrLabel) -> bool {
		*self == CargoVolumePropertyIri || *self == CARGO_VOLUME_PROPERTY_LABEL
	}
}
