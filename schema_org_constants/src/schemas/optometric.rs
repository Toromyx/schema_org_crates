/// <https://schema.org/Optometric>
pub const OPTOMETRIC_IRI_HTTP: &str = "http://schema.org/Optometric";
/// <https://schema.org/Optometric>
pub const OPTOMETRIC_IRI_HTTPS: &str = "https://schema.org/Optometric";
/// <https://schema.org/Optometric>
pub const OPTOMETRIC_LABEL: &str = "Optometric";
pub struct OptometricIri;
impl PartialEq<&str> for OptometricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPTOMETRIC_IRI_HTTP || *other == OPTOMETRIC_IRI_HTTPS
	}
}
impl PartialEq<OptometricIri> for &str {
	fn eq(&self, other: &OptometricIri) -> bool {
		*self == OPTOMETRIC_IRI_HTTP || *self == OPTOMETRIC_IRI_HTTPS
	}
}
pub struct OptometricIriOrLabel;
impl PartialEq<&str> for OptometricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OptometricIri || *other == OPTOMETRIC_LABEL
	}
}
impl PartialEq<OptometricIriOrLabel> for &str {
	fn eq(&self, other: &OptometricIriOrLabel) -> bool {
		*self == OptometricIri || *self == OPTOMETRIC_LABEL
	}
}
