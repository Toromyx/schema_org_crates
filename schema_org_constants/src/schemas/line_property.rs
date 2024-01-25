/// <https://schema.org/line>
pub const LINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/line";
/// <https://schema.org/line>
pub const LINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/line";
/// <https://schema.org/line>
pub const LINE_PROPERTY_LABEL: &str = "line";
pub struct LinePropertyIri;
impl PartialEq<&str> for LinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LINE_PROPERTY_IRI_HTTP || *other == LINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LinePropertyIri> for &str {
	fn eq(&self, other: &LinePropertyIri) -> bool {
		*self == LINE_PROPERTY_IRI_HTTP || *self == LINE_PROPERTY_IRI_HTTPS
	}
}
pub struct LinePropertyIriOrLabel;
impl PartialEq<&str> for LinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LinePropertyIri || *other == LINE_PROPERTY_LABEL
	}
}
impl PartialEq<LinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LinePropertyIriOrLabel) -> bool {
		*self == LinePropertyIri || *self == LINE_PROPERTY_LABEL
	}
}
