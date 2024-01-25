/// <https://schema.org/overdosage>
pub const OVERDOSAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/overdosage";
/// <https://schema.org/overdosage>
pub const OVERDOSAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/overdosage";
/// <https://schema.org/overdosage>
pub const OVERDOSAGE_PROPERTY_LABEL: &str = "overdosage";
pub struct OverdosagePropertyIri;
impl PartialEq<&str> for OverdosagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OVERDOSAGE_PROPERTY_IRI_HTTP || *other == OVERDOSAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OverdosagePropertyIri> for &str {
	fn eq(&self, other: &OverdosagePropertyIri) -> bool {
		*self == OVERDOSAGE_PROPERTY_IRI_HTTP || *self == OVERDOSAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct OverdosagePropertyIriOrLabel;
impl PartialEq<&str> for OverdosagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OverdosagePropertyIri || *other == OVERDOSAGE_PROPERTY_LABEL
	}
}
impl PartialEq<OverdosagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &OverdosagePropertyIriOrLabel) -> bool {
		*self == OverdosagePropertyIri || *self == OVERDOSAGE_PROPERTY_LABEL
	}
}
