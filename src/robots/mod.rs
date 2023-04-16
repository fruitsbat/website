use crate::config::CONFIG;
use cached::proc_macro::cached;

#[cached]
#[get("/robots.txt")]
pub fn robots() -> String {
    format!(
        "{}\n{}",
        include_str!("robots.txt"),
        format!("Sitemap: {}/sitemap.xml", CONFIG.base_url)
    )
}
