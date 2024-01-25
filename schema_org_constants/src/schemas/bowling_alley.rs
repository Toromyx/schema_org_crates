/// <https://schema.org/BowlingAlley>
pub const BOWLING_ALLEY_IRI_HTTP: &str = "http://schema.org/BowlingAlley";
/// <https://schema.org/BowlingAlley>
pub const BOWLING_ALLEY_IRI_HTTPS: &str = "https://schema.org/BowlingAlley";
/// <https://schema.org/BowlingAlley>
pub const BOWLING_ALLEY_LABEL: &str = "BowlingAlley";
pub struct BowlingAlleyIri;
impl PartialEq<&str> for BowlingAlleyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOWLING_ALLEY_IRI_HTTP || *other == BOWLING_ALLEY_IRI_HTTPS
	}
}
impl PartialEq<BowlingAlleyIri> for &str {
	fn eq(&self, other: &BowlingAlleyIri) -> bool {
		*self == BOWLING_ALLEY_IRI_HTTP || *self == BOWLING_ALLEY_IRI_HTTPS
	}
}
pub struct BowlingAlleyIriOrLabel;
impl PartialEq<&str> for BowlingAlleyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BowlingAlleyIri || *other == BOWLING_ALLEY_LABEL
	}
}
impl PartialEq<BowlingAlleyIriOrLabel> for &str {
	fn eq(&self, other: &BowlingAlleyIriOrLabel) -> bool {
		*self == BowlingAlleyIri || *self == BOWLING_ALLEY_LABEL
	}
}
