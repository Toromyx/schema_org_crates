/// <https://schema.org/speed>
pub const SPEED_PROPERTY_IRI_HTTP: &str = "http://schema.org/speed";
/// <https://schema.org/speed>
pub const SPEED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/speed";
/// <https://schema.org/speed>
pub const SPEED_PROPERTY_LABEL: &str = "speed";
pub struct SpeedPropertyIri;
impl PartialEq<&str> for SpeedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPEED_PROPERTY_IRI_HTTP || *other == SPEED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpeedPropertyIri> for &str {
	fn eq(&self, other: &SpeedPropertyIri) -> bool {
		*self == SPEED_PROPERTY_IRI_HTTP || *self == SPEED_PROPERTY_IRI_HTTPS
	}
}
pub struct SpeedPropertyIriOrLabel;
impl PartialEq<&str> for SpeedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpeedPropertyIri || *other == SPEED_PROPERTY_LABEL
	}
}
impl PartialEq<SpeedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpeedPropertyIriOrLabel) -> bool {
		*self == SpeedPropertyIri || *self == SPEED_PROPERTY_LABEL
	}
}
