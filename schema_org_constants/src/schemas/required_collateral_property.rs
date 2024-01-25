/// <https://schema.org/requiredCollateral>
pub const REQUIRED_COLLATERAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiredCollateral";
/// <https://schema.org/requiredCollateral>
pub const REQUIRED_COLLATERAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requiredCollateral";
/// <https://schema.org/requiredCollateral>
pub const REQUIRED_COLLATERAL_PROPERTY_LABEL: &str = "requiredCollateral";
pub struct RequiredCollateralPropertyIri;
impl PartialEq<&str> for RequiredCollateralPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRED_COLLATERAL_PROPERTY_IRI_HTTP
			|| *other == REQUIRED_COLLATERAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiredCollateralPropertyIri> for &str {
	fn eq(&self, other: &RequiredCollateralPropertyIri) -> bool {
		*self == REQUIRED_COLLATERAL_PROPERTY_IRI_HTTP
			|| *self == REQUIRED_COLLATERAL_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiredCollateralPropertyIriOrLabel;
impl PartialEq<&str> for RequiredCollateralPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiredCollateralPropertyIri || *other == REQUIRED_COLLATERAL_PROPERTY_LABEL
	}
}
impl PartialEq<RequiredCollateralPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiredCollateralPropertyIriOrLabel) -> bool {
		*self == RequiredCollateralPropertyIri || *self == REQUIRED_COLLATERAL_PROPERTY_LABEL
	}
}
