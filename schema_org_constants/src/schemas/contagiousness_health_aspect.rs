/// <https://schema.org/ContagiousnessHealthAspect>
pub const CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/ContagiousnessHealthAspect";
/// <https://schema.org/ContagiousnessHealthAspect>
pub const CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/ContagiousnessHealthAspect";
/// <https://schema.org/ContagiousnessHealthAspect>
pub const CONTAGIOUSNESS_HEALTH_ASPECT_LABEL: &str = "ContagiousnessHealthAspect";
pub struct ContagiousnessHealthAspectIri;
impl PartialEq<&str> for ContagiousnessHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTP
			|| *other == CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<ContagiousnessHealthAspectIri> for &str {
	fn eq(&self, other: &ContagiousnessHealthAspectIri) -> bool {
		*self == CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTP
			|| *self == CONTAGIOUSNESS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct ContagiousnessHealthAspectIriOrLabel;
impl PartialEq<&str> for ContagiousnessHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContagiousnessHealthAspectIri || *other == CONTAGIOUSNESS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<ContagiousnessHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &ContagiousnessHealthAspectIriOrLabel) -> bool {
		*self == ContagiousnessHealthAspectIri || *self == CONTAGIOUSNESS_HEALTH_ASPECT_LABEL
	}
}
