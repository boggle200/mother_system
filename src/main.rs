mod user_space_manager;

use user_space_manager::create_user;

fn main() {
    println!("Hello, world!");

    create_user::USER::create_user_space("boggle200");
    create_user::USER::create_user_space("test200");
}
