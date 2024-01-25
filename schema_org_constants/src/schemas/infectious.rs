/// <https://schema.org/Infectious>
pub const INFECTIOUS_IRI_HTTP: &str = "http://schema.org/Infectious";
/// <https://schema.org/Infectious>
pub const INFECTIOUS_IRI_HTTPS: &str = "https://schema.org/Infectious";
/// <https://schema.org/Infectious>
pub const INFECTIOUS_LABEL: &str = "Infectious";
pub struct InfectiousIri;
impl PartialEq<&str> for InfectiousIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFECTIOUS_IRI_HTTP || *other == INFECTIOUS_IRI_HTTPS
	}
}
impl PartialEq<InfectiousIri> for &str {
	fn eq(&self, other: &InfectiousIri) -> bool {
		*self == INFECTIOUS_IRI_HTTP || *self == INFECTIOUS_IRI_HTTPS
	}
}
pub struct InfectiousIriOrLabel;
impl PartialEq<&str> for InfectiousIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InfectiousIri || *other == INFECTIOUS_LABEL
	}
}
impl PartialEq<InfectiousIriOrLabel> for &str {
	fn eq(&self, other: &InfectiousIriOrLabel) -> bool {
		*self == InfectiousIri || *self == INFECTIOUS_LABEL
	}
}
