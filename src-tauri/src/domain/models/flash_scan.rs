use regex::RegexSet;

pub struct FlashScanCategory {
    pub name: String,
    pub paths: Vec<String>,
    pub regexp: Option<RegexSet>,
    pub sub_categories: Option<Vec<FlashScanCategory>>,
}
