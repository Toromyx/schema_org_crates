/// <https://schema.org/isAccessoryOrSparePartFor>
pub const IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isAccessoryOrSparePartFor";
/// <https://schema.org/isAccessoryOrSparePartFor>
pub const IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isAccessoryOrSparePartFor";
/// <https://schema.org/isAccessoryOrSparePartFor>
pub const IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_LABEL: &str = "isAccessoryOrSparePartFor";
pub struct IsAccessoryOrSparePartForPropertyIri;
impl PartialEq<&str> for IsAccessoryOrSparePartForPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTP
			|| *other == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsAccessoryOrSparePartForPropertyIri> for &str {
	fn eq(&self, other: &IsAccessoryOrSparePartForPropertyIri) -> bool {
		*self == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTP
			|| *self == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_IRI_HTTPS
	}
}
pub struct IsAccessoryOrSparePartForPropertyIriOrLabel;
impl PartialEq<&str> for IsAccessoryOrSparePartForPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsAccessoryOrSparePartForPropertyIri
			|| *other == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_LABEL
	}
}
impl PartialEq<IsAccessoryOrSparePartForPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsAccessoryOrSparePartForPropertyIriOrLabel) -> bool {
		*self == IsAccessoryOrSparePartForPropertyIri
			|| *self == IS_ACCESSORY_OR_SPARE_PART_FOR_PROPERTY_LABEL
	}
}
