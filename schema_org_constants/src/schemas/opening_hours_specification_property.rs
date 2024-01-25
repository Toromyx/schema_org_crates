/// <https://schema.org/openingHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/openingHoursSpecification";
/// <https://schema.org/openingHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/openingHoursSpecification";
/// <https://schema.org/openingHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL: &str = "openingHoursSpecification";
pub struct OpeningHoursSpecificationPropertyIri;
impl PartialEq<&str> for OpeningHoursSpecificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *other == OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OpeningHoursSpecificationPropertyIri> for &str {
	fn eq(&self, other: &OpeningHoursSpecificationPropertyIri) -> bool {
		*self == OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *self == OPENING_HOURS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct OpeningHoursSpecificationPropertyIriOrLabel;
impl PartialEq<&str> for OpeningHoursSpecificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpeningHoursSpecificationPropertyIri
			|| *other == OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<OpeningHoursSpecificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OpeningHoursSpecificationPropertyIriOrLabel) -> bool {
		*self == OpeningHoursSpecificationPropertyIri
			|| *self == OPENING_HOURS_SPECIFICATION_PROPERTY_LABEL
	}
}
