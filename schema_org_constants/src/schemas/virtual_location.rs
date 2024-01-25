/// <https://schema.org/VirtualLocation>
pub const VIRTUAL_LOCATION_IRI_HTTP: &str = "http://schema.org/VirtualLocation";
/// <https://schema.org/VirtualLocation>
pub const VIRTUAL_LOCATION_IRI_HTTPS: &str = "https://schema.org/VirtualLocation";
/// <https://schema.org/VirtualLocation>
pub const VIRTUAL_LOCATION_LABEL: &str = "VirtualLocation";
pub struct VirtualLocationIri;
impl PartialEq<&str> for VirtualLocationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIRTUAL_LOCATION_IRI_HTTP || *other == VIRTUAL_LOCATION_IRI_HTTPS
	}
}
impl PartialEq<VirtualLocationIri> for &str {
	fn eq(&self, other: &VirtualLocationIri) -> bool {
		*self == VIRTUAL_LOCATION_IRI_HTTP || *self == VIRTUAL_LOCATION_IRI_HTTPS
	}
}
pub struct VirtualLocationIriOrLabel;
impl PartialEq<&str> for VirtualLocationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VirtualLocationIri || *other == VIRTUAL_LOCATION_LABEL
	}
}
impl PartialEq<VirtualLocationIriOrLabel> for &str {
	fn eq(&self, other: &VirtualLocationIriOrLabel) -> bool {
		*self == VirtualLocationIri || *self == VIRTUAL_LOCATION_LABEL
	}
}
