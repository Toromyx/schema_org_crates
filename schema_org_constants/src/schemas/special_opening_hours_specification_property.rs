/// <https://schema.org/specialOpeningHoursSpecification>
pub const SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/specialOpeningHoursSpecification";
/// <https://schema.org/specialOpeningHoursSpecification>
pub const SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/specialOpeningHoursSpecification";
/// <https://schema.org/specialOpeningHoursSpecification>
pub const SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL: &str =
	"specialOpeningHoursSpecification";
pub struct SpecialOpeningHoursSpecificationPropertyIri;
impl PartialEq<&str> for SpecialOpeningHoursSpecificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *other == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpecialOpeningHoursSpecificationPropertyIri> for &str {
	fn eq(&self, other: &SpecialOpeningHoursSpecificationPropertyIri) -> bool {
		*self == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *self == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SpecialOpeningHoursSpecificationPropertyIriOrLabel;
impl PartialEq<&str> for SpecialOpeningHoursSpecificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpecialOpeningHoursSpecificationPropertyIri
			|| *other == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<SpecialOpeningHoursSpecificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpecialOpeningHoursSpecificationPropertyIriOrLabel) -> bool {
		*self == SpecialOpeningHoursSpecificationPropertyIri
			|| *self == SPECIAL_OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL
	}
}
