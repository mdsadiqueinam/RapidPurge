use regex::RegexSet;
#[derive(Clone, Debug)]
pub struct FlashScanCategory {
    pub id: String,
    pub name: String,
    pub priority: u32,
    pub paths: Vec<String>,
    pub regexp: Option<RegexSet>,
    pub sub_categories: Option<Vec<FlashScanCategory>>,
}
