/// <https://schema.org/WeaponConsideration>
pub const WEAPON_CONSIDERATION_IRI_HTTP: &str = "http://schema.org/WeaponConsideration";
/// <https://schema.org/WeaponConsideration>
pub const WEAPON_CONSIDERATION_IRI_HTTPS: &str = "https://schema.org/WeaponConsideration";
/// <https://schema.org/WeaponConsideration>
pub const WEAPON_CONSIDERATION_LABEL: &str = "WeaponConsideration";
pub struct WeaponConsiderationIri;
impl PartialEq<&str> for WeaponConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEAPON_CONSIDERATION_IRI_HTTP || *other == WEAPON_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<WeaponConsiderationIri> for &str {
	fn eq(&self, other: &WeaponConsiderationIri) -> bool {
		*self == WEAPON_CONSIDERATION_IRI_HTTP || *self == WEAPON_CONSIDERATION_IRI_HTTPS
	}
}
pub struct WeaponConsiderationIriOrLabel;
impl PartialEq<&str> for WeaponConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WeaponConsiderationIri || *other == WEAPON_CONSIDERATION_LABEL
	}
}
impl PartialEq<WeaponConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &WeaponConsiderationIriOrLabel) -> bool {
		*self == WeaponConsiderationIri || *self == WEAPON_CONSIDERATION_LABEL
	}
}
