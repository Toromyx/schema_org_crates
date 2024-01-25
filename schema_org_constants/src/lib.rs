mod schemas;
pub use self::schemas::*;

#[derive(Debug)]
pub enum SchemaOrgNamespace {
	Http,
	Https,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_partial_eq() {
		assert!(RecipeIri == "http://schema.org/Recipe");
		assert!(RecipeIri == "https://schema.org/Recipe");
		assert!("http://schema.org/Recipe" == RecipeIri);
		assert!("https://schema.org/Recipe" == RecipeIri);
		assert!(RecipeIriOrLabel == "http://schema.org/Recipe");
		assert!(RecipeIriOrLabel == "https://schema.org/Recipe");
		assert!(RecipeIriOrLabel == "Recipe");
		assert!("http://schema.org/Recipe" == RecipeIriOrLabel);
		assert!("https://schema.org/Recipe" == RecipeIriOrLabel);
		assert!("Recipe" == RecipeIriOrLabel);
		assert!(RecipeIri != "ftp://schema.org/Recipe");
		assert!("ftps://schema.org/Recipe" != RecipeIri);
		assert!(RecipeIriOrLabel != "Recipes");
		assert!("recipe" != RecipeIriOrLabel);
	}
}
