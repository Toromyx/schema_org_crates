/// <https://schema.org/globalLocationNumber>
pub const GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/globalLocationNumber";
/// <https://schema.org/globalLocationNumber>
pub const GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/globalLocationNumber";
/// <https://schema.org/globalLocationNumber>
pub const GLOBAL_LOCATION_NUMBER_PROPERTY_LABEL: &str = "globalLocationNumber";
pub struct GlobalLocationNumberPropertyIri;
impl PartialEq<&str> for GlobalLocationNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTP
			|| *other == GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GlobalLocationNumberPropertyIri> for &str {
	fn eq(&self, other: &GlobalLocationNumberPropertyIri) -> bool {
		*self == GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTP
			|| *self == GLOBAL_LOCATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct GlobalLocationNumberPropertyIriOrLabel;
impl PartialEq<&str> for GlobalLocationNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GlobalLocationNumberPropertyIri || *other == GLOBAL_LOCATION_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<GlobalLocationNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GlobalLocationNumberPropertyIriOrLabel) -> bool {
		*self == GlobalLocationNumberPropertyIri || *self == GLOBAL_LOCATION_NUMBER_PROPERTY_LABEL
	}
}
