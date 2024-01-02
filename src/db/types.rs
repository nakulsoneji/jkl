#[derive(sqlx::FromRow, Clone)]
pub struct PData {
    pub name: String,
    pub ver: String,
}

impl PData {
    pub(crate) fn new(n: &str, v: &str) -> PData {
        PData {
            name: String::from(n),
            ver: String::from(v),
        }
    }
    pub(crate) fn folder_name(&self) -> String {
        format!("{}-{}", &self.name, &self.ver)
    }
}
