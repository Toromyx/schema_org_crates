/// <https://schema.org/wheelbase>
pub const WHEELBASE_PROPERTY_IRI_HTTP: &str = "http://schema.org/wheelbase";
/// <https://schema.org/wheelbase>
pub const WHEELBASE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/wheelbase";
/// <https://schema.org/wheelbase>
pub const WHEELBASE_PROPERTY_LABEL: &str = "wheelbase";
pub struct WheelbasePropertyIri;
impl PartialEq<&str> for WheelbasePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WHEELBASE_PROPERTY_IRI_HTTP || *other == WHEELBASE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WheelbasePropertyIri> for &str {
	fn eq(&self, other: &WheelbasePropertyIri) -> bool {
		*self == WHEELBASE_PROPERTY_IRI_HTTP || *self == WHEELBASE_PROPERTY_IRI_HTTPS
	}
}
pub struct WheelbasePropertyIriOrLabel;
impl PartialEq<&str> for WheelbasePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WheelbasePropertyIri || *other == WHEELBASE_PROPERTY_LABEL
	}
}
impl PartialEq<WheelbasePropertyIriOrLabel> for &str {
	fn eq(&self, other: &WheelbasePropertyIriOrLabel) -> bool {
		*self == WheelbasePropertyIri || *self == WHEELBASE_PROPERTY_LABEL
	}
}
