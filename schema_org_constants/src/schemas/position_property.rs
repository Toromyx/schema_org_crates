/// <https://schema.org/position>
pub const POSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/position";
/// <https://schema.org/position>
pub const POSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/position";
/// <https://schema.org/position>
pub const POSITION_PROPERTY_LABEL: &str = "position";
pub struct PositionPropertyIri;
impl PartialEq<&str> for PositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSITION_PROPERTY_IRI_HTTP || *other == POSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PositionPropertyIri> for &str {
	fn eq(&self, other: &PositionPropertyIri) -> bool {
		*self == POSITION_PROPERTY_IRI_HTTP || *self == POSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct PositionPropertyIriOrLabel;
impl PartialEq<&str> for PositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PositionPropertyIri || *other == POSITION_PROPERTY_LABEL
	}
}
impl PartialEq<PositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PositionPropertyIriOrLabel) -> bool {
		*self == PositionPropertyIri || *self == POSITION_PROPERTY_LABEL
	}
}
