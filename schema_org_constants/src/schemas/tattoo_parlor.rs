/// <https://schema.org/TattooParlor>
pub const TATTOO_PARLOR_IRI_HTTP: &str = "http://schema.org/TattooParlor";
/// <https://schema.org/TattooParlor>
pub const TATTOO_PARLOR_IRI_HTTPS: &str = "https://schema.org/TattooParlor";
/// <https://schema.org/TattooParlor>
pub const TATTOO_PARLOR_LABEL: &str = "TattooParlor";
pub struct TattooParlorIri;
impl PartialEq<&str> for TattooParlorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TATTOO_PARLOR_IRI_HTTP || *other == TATTOO_PARLOR_IRI_HTTPS
	}
}
impl PartialEq<TattooParlorIri> for &str {
	fn eq(&self, other: &TattooParlorIri) -> bool {
		*self == TATTOO_PARLOR_IRI_HTTP || *self == TATTOO_PARLOR_IRI_HTTPS
	}
}
pub struct TattooParlorIriOrLabel;
impl PartialEq<&str> for TattooParlorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TattooParlorIri || *other == TATTOO_PARLOR_LABEL
	}
}
impl PartialEq<TattooParlorIriOrLabel> for &str {
	fn eq(&self, other: &TattooParlorIriOrLabel) -> bool {
		*self == TattooParlorIri || *self == TATTOO_PARLOR_LABEL
	}
}
