/// <https://schema.org/isConsumableFor>
pub const IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/isConsumableFor";
/// <https://schema.org/isConsumableFor>
pub const IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isConsumableFor";
/// <https://schema.org/isConsumableFor>
pub const IS_CONSUMABLE_FOR_PROPERTY_LABEL: &str = "isConsumableFor";
pub struct IsConsumableForPropertyIri;
impl PartialEq<&str> for IsConsumableForPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTP
			|| *other == IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsConsumableForPropertyIri> for &str {
	fn eq(&self, other: &IsConsumableForPropertyIri) -> bool {
		*self == IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTP
			|| *self == IS_CONSUMABLE_FOR_PROPERTY_IRI_HTTPS
	}
}
pub struct IsConsumableForPropertyIriOrLabel;
impl PartialEq<&str> for IsConsumableForPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsConsumableForPropertyIri || *other == IS_CONSUMABLE_FOR_PROPERTY_LABEL
	}
}
impl PartialEq<IsConsumableForPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsConsumableForPropertyIriOrLabel) -> bool {
		*self == IsConsumableForPropertyIri || *self == IS_CONSUMABLE_FOR_PROPERTY_LABEL
	}
}
