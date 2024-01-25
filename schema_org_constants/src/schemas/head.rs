/// <https://schema.org/Head>
pub const HEAD_IRI_HTTP: &str = "http://schema.org/Head";
/// <https://schema.org/Head>
pub const HEAD_IRI_HTTPS: &str = "https://schema.org/Head";
/// <https://schema.org/Head>
pub const HEAD_LABEL: &str = "Head";
pub struct HeadIri;
impl PartialEq<&str> for HeadIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEAD_IRI_HTTP || *other == HEAD_IRI_HTTPS
	}
}
impl PartialEq<HeadIri> for &str {
	fn eq(&self, other: &HeadIri) -> bool {
		*self == HEAD_IRI_HTTP || *self == HEAD_IRI_HTTPS
	}
}
pub struct HeadIriOrLabel;
impl PartialEq<&str> for HeadIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HeadIri || *other == HEAD_LABEL
	}
}
impl PartialEq<HeadIriOrLabel> for &str {
	fn eq(&self, other: &HeadIriOrLabel) -> bool {
		*self == HeadIri || *self == HEAD_LABEL
	}
}
