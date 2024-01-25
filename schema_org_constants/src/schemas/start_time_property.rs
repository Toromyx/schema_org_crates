/// <https://schema.org/startTime>
pub const START_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/startTime";
/// <https://schema.org/startTime>
pub const START_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/startTime";
/// <https://schema.org/startTime>
pub const START_TIME_PROPERTY_LABEL: &str = "startTime";
pub struct StartTimePropertyIri;
impl PartialEq<&str> for StartTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == START_TIME_PROPERTY_IRI_HTTP || *other == START_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StartTimePropertyIri> for &str {
	fn eq(&self, other: &StartTimePropertyIri) -> bool {
		*self == START_TIME_PROPERTY_IRI_HTTP || *self == START_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct StartTimePropertyIriOrLabel;
impl PartialEq<&str> for StartTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StartTimePropertyIri || *other == START_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<StartTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StartTimePropertyIriOrLabel) -> bool {
		*self == StartTimePropertyIri || *self == START_TIME_PROPERTY_LABEL
	}
}
