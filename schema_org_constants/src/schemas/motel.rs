/// <https://schema.org/Motel>
pub const MOTEL_IRI_HTTP: &str = "http://schema.org/Motel";
/// <https://schema.org/Motel>
pub const MOTEL_IRI_HTTPS: &str = "https://schema.org/Motel";
/// <https://schema.org/Motel>
pub const MOTEL_LABEL: &str = "Motel";
pub struct MotelIri;
impl PartialEq<&str> for MotelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOTEL_IRI_HTTP || *other == MOTEL_IRI_HTTPS
	}
}
impl PartialEq<MotelIri> for &str {
	fn eq(&self, other: &MotelIri) -> bool {
		*self == MOTEL_IRI_HTTP || *self == MOTEL_IRI_HTTPS
	}
}
pub struct MotelIriOrLabel;
impl PartialEq<&str> for MotelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MotelIri || *other == MOTEL_LABEL
	}
}
impl PartialEq<MotelIriOrLabel> for &str {
	fn eq(&self, other: &MotelIriOrLabel) -> bool {
		*self == MotelIri || *self == MOTEL_LABEL
	}
}
