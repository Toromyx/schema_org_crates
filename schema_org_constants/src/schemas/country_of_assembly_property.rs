/// <https://schema.org/countryOfAssembly>
pub const COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/countryOfAssembly";
/// <https://schema.org/countryOfAssembly>
pub const COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/countryOfAssembly";
/// <https://schema.org/countryOfAssembly>
pub const COUNTRY_OF_ASSEMBLY_PROPERTY_LABEL: &str = "countryOfAssembly";
pub struct CountryOfAssemblyPropertyIri;
impl PartialEq<&str> for CountryOfAssemblyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTP
			|| *other == COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CountryOfAssemblyPropertyIri> for &str {
	fn eq(&self, other: &CountryOfAssemblyPropertyIri) -> bool {
		*self == COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTP
			|| *self == COUNTRY_OF_ASSEMBLY_PROPERTY_IRI_HTTPS
	}
}
pub struct CountryOfAssemblyPropertyIriOrLabel;
impl PartialEq<&str> for CountryOfAssemblyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountryOfAssemblyPropertyIri || *other == COUNTRY_OF_ASSEMBLY_PROPERTY_LABEL
	}
}
impl PartialEq<CountryOfAssemblyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CountryOfAssemblyPropertyIriOrLabel) -> bool {
		*self == CountryOfAssemblyPropertyIri || *self == COUNTRY_OF_ASSEMBLY_PROPERTY_LABEL
	}
}
