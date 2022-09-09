use diesel_demo::*;

fn main() {
    let connection = &mut establish_connection();
    create_post(connection, "test", "body");
}
