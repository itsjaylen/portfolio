use super::{
    args::{ UserCommand, UserSubcommand },
    modules::{ create::CreateUser, delete::DeleteEntity, update::UpdateUser },
};

// Handles all the user subcommands
pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(delete_entity) => {
            delete_user(delete_entity);
        }
        UserSubcommand::Show => {
            show_users();
        }
    }
}

// Commands ran handlers

fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);
}

fn delete_user(user: DeleteEntity) {
    println!("Deleting  user: {:?}", user);
}

fn show_users() {
    println!("showing users");
}
