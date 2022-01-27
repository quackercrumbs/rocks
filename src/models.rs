#[derive(Queryable,Debug)]
pub struct ApiResponse {
    pub id: i32,
    pub start_date: String,
    pub end_date: String,
    pub response: String
}

use super::schema::api_response;
#[derive(Insertable)]
#[table_name="api_response"]
pub struct NewApiResponse<'a> {
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub response: &'a str 
}
