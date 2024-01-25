/// <https://schema.org/sha256>
pub const SHA_256_PROPERTY_IRI_HTTP: &str = "http://schema.org/sha256";
/// <https://schema.org/sha256>
pub const SHA_256_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sha256";
/// <https://schema.org/sha256>
pub const SHA_256_PROPERTY_LABEL: &str = "sha256";
pub struct Sha256PropertyIri;
impl PartialEq<&str> for Sha256PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHA_256_PROPERTY_IRI_HTTP || *other == SHA_256_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Sha256PropertyIri> for &str {
	fn eq(&self, other: &Sha256PropertyIri) -> bool {
		*self == SHA_256_PROPERTY_IRI_HTTP || *self == SHA_256_PROPERTY_IRI_HTTPS
	}
}
pub struct Sha256PropertyIriOrLabel;
impl PartialEq<&str> for Sha256PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Sha256PropertyIri || *other == SHA_256_PROPERTY_LABEL
	}
}
impl PartialEq<Sha256PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Sha256PropertyIriOrLabel) -> bool {
		*self == Sha256PropertyIri || *self == SHA_256_PROPERTY_LABEL
	}
}
