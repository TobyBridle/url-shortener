use super::slug_repository;

pub struct SlugRepository;
impl slug_repository::SlugRepository for SlugRepository {
    fn get_slug(slug: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Sample Slug".to_string())
    }

    fn insert_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok("test slug".to_string())
    }
}