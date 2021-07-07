use crate::db::Db;
use crate::db::error::DbError;

pub async fn get_rand_line_from_db(filename: &str) -> Result<String, DbError> {
    let db = Db::establish(&filename).await?;
    let line = db.get_rand_line().await?;
    Ok(line)
}