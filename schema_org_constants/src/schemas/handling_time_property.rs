/// <https://schema.org/handlingTime>
pub const HANDLING_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/handlingTime";
/// <https://schema.org/handlingTime>
pub const HANDLING_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/handlingTime";
/// <https://schema.org/handlingTime>
pub const HANDLING_TIME_PROPERTY_LABEL: &str = "handlingTime";
pub struct HandlingTimePropertyIri;
impl PartialEq<&str> for HandlingTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HANDLING_TIME_PROPERTY_IRI_HTTP || *other == HANDLING_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HandlingTimePropertyIri> for &str {
	fn eq(&self, other: &HandlingTimePropertyIri) -> bool {
		*self == HANDLING_TIME_PROPERTY_IRI_HTTP || *self == HANDLING_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct HandlingTimePropertyIriOrLabel;
impl PartialEq<&str> for HandlingTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HandlingTimePropertyIri || *other == HANDLING_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<HandlingTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HandlingTimePropertyIriOrLabel) -> bool {
		*self == HandlingTimePropertyIri || *self == HANDLING_TIME_PROPERTY_LABEL
	}
}
