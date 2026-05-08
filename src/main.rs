use scopa_fish::game;

fn main() {
    game::game_loop(
        game::GameInfo {
            display_all_debug: false,
            display_debug: true,
        },
        game::get_input,
        game::get_input,
    );
}
