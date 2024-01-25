/// <https://schema.org/numAdults>
pub const NUM_ADULTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numAdults";
/// <https://schema.org/numAdults>
pub const NUM_ADULTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numAdults";
/// <https://schema.org/numAdults>
pub const NUM_ADULTS_PROPERTY_LABEL: &str = "numAdults";
pub struct NumAdultsPropertyIri;
impl PartialEq<&str> for NumAdultsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUM_ADULTS_PROPERTY_IRI_HTTP || *other == NUM_ADULTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumAdultsPropertyIri> for &str {
	fn eq(&self, other: &NumAdultsPropertyIri) -> bool {
		*self == NUM_ADULTS_PROPERTY_IRI_HTTP || *self == NUM_ADULTS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumAdultsPropertyIriOrLabel;
impl PartialEq<&str> for NumAdultsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumAdultsPropertyIri || *other == NUM_ADULTS_PROPERTY_LABEL
	}
}
impl PartialEq<NumAdultsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumAdultsPropertyIriOrLabel) -> bool {
		*self == NumAdultsPropertyIri || *self == NUM_ADULTS_PROPERTY_LABEL
	}
}
