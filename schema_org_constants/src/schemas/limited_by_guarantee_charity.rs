/// <https://schema.org/LimitedByGuaranteeCharity>
pub const LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTP: &str =
	"http://schema.org/LimitedByGuaranteeCharity";
/// <https://schema.org/LimitedByGuaranteeCharity>
pub const LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTPS: &str =
	"https://schema.org/LimitedByGuaranteeCharity";
/// <https://schema.org/LimitedByGuaranteeCharity>
pub const LIMITED_BY_GUARANTEE_CHARITY_LABEL: &str = "LimitedByGuaranteeCharity";
pub struct LimitedByGuaranteeCharityIri;
impl PartialEq<&str> for LimitedByGuaranteeCharityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTP
			|| *other == LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTPS
	}
}
impl PartialEq<LimitedByGuaranteeCharityIri> for &str {
	fn eq(&self, other: &LimitedByGuaranteeCharityIri) -> bool {
		*self == LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTP
			|| *self == LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTPS
	}
}
pub struct LimitedByGuaranteeCharityIriOrLabel;
impl PartialEq<&str> for LimitedByGuaranteeCharityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LimitedByGuaranteeCharityIri || *other == LIMITED_BY_GUARANTEE_CHARITY_LABEL
	}
}
impl PartialEq<LimitedByGuaranteeCharityIriOrLabel> for &str {
	fn eq(&self, other: &LimitedByGuaranteeCharityIriOrLabel) -> bool {
		*self == LimitedByGuaranteeCharityIri || *self == LIMITED_BY_GUARANTEE_CHARITY_LABEL
	}
}
