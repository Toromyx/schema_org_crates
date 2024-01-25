/// <https://schema.org/numberedPosition>
pub const NUMBERED_POSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberedPosition";
/// <https://schema.org/numberedPosition>
pub const NUMBERED_POSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberedPosition";
/// <https://schema.org/numberedPosition>
pub const NUMBERED_POSITION_PROPERTY_LABEL: &str = "numberedPosition";
pub struct NumberedPositionPropertyIri;
impl PartialEq<&str> for NumberedPositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBERED_POSITION_PROPERTY_IRI_HTTP
			|| *other == NUMBERED_POSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberedPositionPropertyIri> for &str {
	fn eq(&self, other: &NumberedPositionPropertyIri) -> bool {
		*self == NUMBERED_POSITION_PROPERTY_IRI_HTTP
			|| *self == NUMBERED_POSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberedPositionPropertyIriOrLabel;
impl PartialEq<&str> for NumberedPositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberedPositionPropertyIri || *other == NUMBERED_POSITION_PROPERTY_LABEL
	}
}
impl PartialEq<NumberedPositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberedPositionPropertyIriOrLabel) -> bool {
		*self == NumberedPositionPropertyIri || *self == NUMBERED_POSITION_PROPERTY_LABEL
	}
}
