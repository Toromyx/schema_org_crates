/// <https://schema.org/startOffset>
pub const START_OFFSET_PROPERTY_IRI_HTTP: &str = "http://schema.org/startOffset";
/// <https://schema.org/startOffset>
pub const START_OFFSET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/startOffset";
/// <https://schema.org/startOffset>
pub const START_OFFSET_PROPERTY_LABEL: &str = "startOffset";
pub struct StartOffsetPropertyIri;
impl PartialEq<&str> for StartOffsetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == START_OFFSET_PROPERTY_IRI_HTTP || *other == START_OFFSET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StartOffsetPropertyIri> for &str {
	fn eq(&self, other: &StartOffsetPropertyIri) -> bool {
		*self == START_OFFSET_PROPERTY_IRI_HTTP || *self == START_OFFSET_PROPERTY_IRI_HTTPS
	}
}
pub struct StartOffsetPropertyIriOrLabel;
impl PartialEq<&str> for StartOffsetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StartOffsetPropertyIri || *other == START_OFFSET_PROPERTY_LABEL
	}
}
impl PartialEq<StartOffsetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StartOffsetPropertyIriOrLabel) -> bool {
		*self == StartOffsetPropertyIri || *self == START_OFFSET_PROPERTY_LABEL
	}
}
