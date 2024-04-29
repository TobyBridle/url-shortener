pub trait SlugRepository {
    /// Return the URL for the slug, or an Error if
    /// either the slug does not have a corresponding URL
    /// or an internal error occurred.
    fn get_expanded_url(slug: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Store the URL and get the resulting slug or return
    /// an Error if the insertion failed. Duplicate URLs should
    /// receive separate slugs.
    fn insert_url(url: String) -> Result<String, Box<dyn std::error::Error>>;
}