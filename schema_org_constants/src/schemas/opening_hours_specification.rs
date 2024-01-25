/// <https://schema.org/OpeningHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/OpeningHoursSpecification";
/// <https://schema.org/OpeningHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/OpeningHoursSpecification";
/// <https://schema.org/OpeningHoursSpecification>
pub const OPENING_HOURS_SPECIFICATION_LABEL: &str = "OpeningHoursSpecification";
pub struct OpeningHoursSpecificationIri;
impl PartialEq<&str> for OpeningHoursSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPENING_HOURS_SPECIFICATION_IRI_HTTP
			|| *other == OPENING_HOURS_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<OpeningHoursSpecificationIri> for &str {
	fn eq(&self, other: &OpeningHoursSpecificationIri) -> bool {
		*self == OPENING_HOURS_SPECIFICATION_IRI_HTTP
			|| *self == OPENING_HOURS_SPECIFICATION_IRI_HTTPS
	}
}
pub struct OpeningHoursSpecificationIriOrLabel;
impl PartialEq<&str> for OpeningHoursSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpeningHoursSpecificationIri || *other == OPENING_HOURS_SPECIFICATION_LABEL
	}
}
impl PartialEq<OpeningHoursSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &OpeningHoursSpecificationIriOrLabel) -> bool {
		*self == OpeningHoursSpecificationIri || *self == OPENING_HOURS_SPECIFICATION_LABEL
	}
}
