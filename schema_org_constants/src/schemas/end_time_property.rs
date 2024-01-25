/// <https://schema.org/endTime>
pub const END_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/endTime";
/// <https://schema.org/endTime>
pub const END_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/endTime";
/// <https://schema.org/endTime>
pub const END_TIME_PROPERTY_LABEL: &str = "endTime";
pub struct EndTimePropertyIri;
impl PartialEq<&str> for EndTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == END_TIME_PROPERTY_IRI_HTTP || *other == END_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EndTimePropertyIri> for &str {
	fn eq(&self, other: &EndTimePropertyIri) -> bool {
		*self == END_TIME_PROPERTY_IRI_HTTP || *self == END_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct EndTimePropertyIriOrLabel;
impl PartialEq<&str> for EndTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndTimePropertyIri || *other == END_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<EndTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EndTimePropertyIriOrLabel) -> bool {
		*self == EndTimePropertyIri || *self == END_TIME_PROPERTY_LABEL
	}
}
