/// <https://schema.org/MRI>
pub const MRI_IRI_HTTP: &str = "http://schema.org/MRI";
/// <https://schema.org/MRI>
pub const MRI_IRI_HTTPS: &str = "https://schema.org/MRI";
/// <https://schema.org/MRI>
pub const MRI_LABEL: &str = "MRI";
pub struct MriIri;
impl PartialEq<&str> for MriIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MRI_IRI_HTTP || *other == MRI_IRI_HTTPS
	}
}
impl PartialEq<MriIri> for &str {
	fn eq(&self, other: &MriIri) -> bool {
		*self == MRI_IRI_HTTP || *self == MRI_IRI_HTTPS
	}
}
pub struct MriIriOrLabel;
impl PartialEq<&str> for MriIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MriIri || *other == MRI_LABEL
	}
}
impl PartialEq<MriIriOrLabel> for &str {
	fn eq(&self, other: &MriIriOrLabel) -> bool {
		*self == MriIri || *self == MRI_LABEL
	}
}
