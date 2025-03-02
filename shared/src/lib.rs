use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecipe {
    pub title: String,
    pub description: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

// API Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

// Server function results
pub type ServerResult<T> = Result<T, String>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
