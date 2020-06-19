pub mod users;

use rocket_contrib;
use rocket_contrib::databases::diesel;


#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);
