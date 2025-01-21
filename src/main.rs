mod course_page;
mod courses;
mod env;
mod menu;

use env::Env;
use menu::Menu;

#[tokio::main]
async fn main() {
    let env = Env::get();
    let courses = courses::get_course(&env).await;

    let mut terminal = ratatui::init();
    let menu_result = Menu::default(courses).run(&mut terminal);
    ratatui::restore();
    menu_result.expect("TODO!");
}
