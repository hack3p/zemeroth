use hate::{self, Context, Event, Screen, Sprite, Time};
use hate::gui::{self, Gui};
use hate::geom::Point;
use screen;

#[derive(Copy, Clone, Debug)]
enum Command {
    Exit,
    Start,
    GuiTest,
    ActionsTest,
}

#[derive(Debug)]
pub struct MainMenu {
    gui: Gui<Command>,
    sprite: Sprite,
}

impl MainMenu {
    pub fn new(context: &mut Context) -> Self {
        let mut gui = Gui::new(context);
        {
            let sprite_exit = gui::text_sprite(context, "exit", 0.1);
            let sprite_start = gui::text_sprite(context, "start", 0.1);
            let sprite_gui_test = gui::text_sprite(context, "gui test", 0.1);
            let sprite_actions_test = gui::text_sprite(context, "actions test", 0.1);
            let button_id_exit = gui.add_button(context, sprite_exit, Command::Exit);
            let button_id_start = gui.add_button(context, sprite_start, Command::Start);
            let button_id_gui_test = gui.add_button(context, sprite_gui_test, Command::GuiTest);
            let button_id_actions_test =
                gui.add_button(context, sprite_actions_test, Command::ActionsTest);
            let anchor = gui::Anchor {
                vertical: gui::VAnchor::Middle,
                horizontal: gui::HAnchor::Middle,
            };
            let direction = gui::Direction::Up;
            let _ = gui.add_layout(
                anchor,
                direction,
                vec![
                    button_id_exit,
                    button_id_actions_test,
                    button_id_gui_test,
                    button_id_start,
                ],
            );
        }
        let mut sprite_imp = Sprite::from_path(context, "imp.png", 2.0);
        sprite_imp.set_color([0.0, 0.0, 1.0, 0.2]);
        MainMenu {
            gui,
            sprite: sprite_imp,
        }
    }

    fn open_screen_gui_test(&mut self, context: &mut Context) {
        let screen = Box::new(screen::GuiTest::new(context));
        context.add_command(hate::screen::Command::Push(screen));
    }

    fn open_screen_actions_test(&mut self, context: &mut Context) {
        let screen = Box::new(screen::ActionsTest::new(context));
        context.add_command(hate::screen::Command::Push(screen));
    }

    fn start_new_game(&mut self, context: &mut Context) {
        let game_screen = Box::new(screen::Game::new(context));
        context.add_command(hate::screen::Command::Push(game_screen));
    }

    fn exit(&mut self, context: &mut Context) {
        context.add_command(hate::screen::Command::Pop);
    }

    fn handle_event_click(&mut self, context: &mut Context, pos: Point) {
        self.gui.click(pos);
        while let Some(command) = self.gui.try_recv() {
            match command {
                Command::GuiTest => self.open_screen_gui_test(context),
                Command::ActionsTest => self.open_screen_actions_test(context),
                Command::Start => self.start_new_game(context),
                Command::Exit => self.exit(context),
            }
        }
    }
}

impl Screen for MainMenu {
    fn tick(&mut self, context: &mut Context, _: Time) {
        let projection_matrix = context.projection_matrix();
        self.sprite.draw(context, projection_matrix);
        self.gui.draw(context);
    }

    fn handle_event(&mut self, context: &mut Context, event: Event) {
        match event {
            Event::Click { pos } => {
                self.handle_event_click(context, pos);
            }
            Event::Resize { aspect_ratio } => {
                self.gui.resize(aspect_ratio);
            }
        }
    }
}
