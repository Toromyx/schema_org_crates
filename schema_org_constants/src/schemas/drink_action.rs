/// <https://schema.org/DrinkAction>
pub const DRINK_ACTION_IRI_HTTP: &str = "http://schema.org/DrinkAction";
/// <https://schema.org/DrinkAction>
pub const DRINK_ACTION_IRI_HTTPS: &str = "https://schema.org/DrinkAction";
/// <https://schema.org/DrinkAction>
pub const DRINK_ACTION_LABEL: &str = "DrinkAction";
pub struct DrinkActionIri;
impl PartialEq<&str> for DrinkActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRINK_ACTION_IRI_HTTP || *other == DRINK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DrinkActionIri> for &str {
	fn eq(&self, other: &DrinkActionIri) -> bool {
		*self == DRINK_ACTION_IRI_HTTP || *self == DRINK_ACTION_IRI_HTTPS
	}
}
pub struct DrinkActionIriOrLabel;
impl PartialEq<&str> for DrinkActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrinkActionIri || *other == DRINK_ACTION_LABEL
	}
}
impl PartialEq<DrinkActionIriOrLabel> for &str {
	fn eq(&self, other: &DrinkActionIriOrLabel) -> bool {
		*self == DrinkActionIri || *self == DRINK_ACTION_LABEL
	}
}
