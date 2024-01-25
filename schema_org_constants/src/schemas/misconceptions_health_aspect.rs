/// <https://schema.org/MisconceptionsHealthAspect>
pub const MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTP: &str =
	"http://schema.org/MisconceptionsHealthAspect";
/// <https://schema.org/MisconceptionsHealthAspect>
pub const MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTPS: &str =
	"https://schema.org/MisconceptionsHealthAspect";
/// <https://schema.org/MisconceptionsHealthAspect>
pub const MISCONCEPTIONS_HEALTH_ASPECT_LABEL: &str = "MisconceptionsHealthAspect";
pub struct MisconceptionsHealthAspectIri;
impl PartialEq<&str> for MisconceptionsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTP
			|| *other == MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<MisconceptionsHealthAspectIri> for &str {
	fn eq(&self, other: &MisconceptionsHealthAspectIri) -> bool {
		*self == MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTP
			|| *self == MISCONCEPTIONS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct MisconceptionsHealthAspectIriOrLabel;
impl PartialEq<&str> for MisconceptionsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MisconceptionsHealthAspectIri || *other == MISCONCEPTIONS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<MisconceptionsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &MisconceptionsHealthAspectIriOrLabel) -> bool {
		*self == MisconceptionsHealthAspectIri || *self == MISCONCEPTIONS_HEALTH_ASPECT_LABEL
	}
}
