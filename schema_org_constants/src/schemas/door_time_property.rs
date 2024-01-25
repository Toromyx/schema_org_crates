/// <https://schema.org/doorTime>
pub const DOOR_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/doorTime";
/// <https://schema.org/doorTime>
pub const DOOR_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/doorTime";
/// <https://schema.org/doorTime>
pub const DOOR_TIME_PROPERTY_LABEL: &str = "doorTime";
pub struct DoorTimePropertyIri;
impl PartialEq<&str> for DoorTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOOR_TIME_PROPERTY_IRI_HTTP || *other == DOOR_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DoorTimePropertyIri> for &str {
	fn eq(&self, other: &DoorTimePropertyIri) -> bool {
		*self == DOOR_TIME_PROPERTY_IRI_HTTP || *self == DOOR_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct DoorTimePropertyIriOrLabel;
impl PartialEq<&str> for DoorTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoorTimePropertyIri || *other == DOOR_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<DoorTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DoorTimePropertyIriOrLabel) -> bool {
		*self == DoorTimePropertyIri || *self == DOOR_TIME_PROPERTY_LABEL
	}
}
