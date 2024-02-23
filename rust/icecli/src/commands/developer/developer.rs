use super::{ args::{ DeveloperCommand, DeveloperSubcommand }, modules::tests::Test };

// Handles all the user subcommands
pub fn handle_developer_command(developer: DeveloperCommand) {
    let command = developer.command;
    match command {
        DeveloperSubcommand::Test(developer) => {
            _test(developer);
        }
    }
}

// Commands ran handlers

fn _test(developer: Test) {
    println!("Test args: {:?}", developer);
}
