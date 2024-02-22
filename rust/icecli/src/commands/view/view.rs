use super::{ args::{ ViewCommand, ViewSubcommand }, modules::create::CreateView };

// Handles all view subcommands
pub fn handle_view_command(view: ViewCommand) {
    let command = view.command;
    match command {
        ViewSubcommand::Create(new_view) => {
            create_view(new_view);
        }
        ViewSubcommand::Show => {
            show_views();
        }
        ViewSubcommand::ShowPretty => {
            show_views_pretty();
        }
    }
}

// Commands ran handlers

fn create_view(new_view: CreateView) {
    println!("Creating view: {:?}", new_view);
}

fn show_views() {
    println!("Showing views");
}

fn show_views_pretty() {
    println!("Showing views");
}
