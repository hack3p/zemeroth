use hate::{self, Context, Event, Screen, /*Sprite,*/ Time};
use hate::geom::Point;
use hate::gui::{self, Gui};

#[derive(Copy, Clone, Debug)]
enum GuiCommand {
    ShowHide,
    Exit,
}

#[derive(Debug)]
pub struct ActionsTest {
    gui: Gui<GuiCommand>,
}

impl ActionsTest {
    pub fn new(context: &mut Context) -> Self {
        let mut gui = Gui::new(context);

        let sprite_show_hide = gui::text_sprite(context, "show/hide", 0.1);
        let sprite_exit = gui::text_sprite(context, "exit", 0.1);
        let button_id_show_hide = gui.add_button(context, sprite_show_hide, GuiCommand::ShowHide);
        let button_id_exit = gui.add_button(context, sprite_exit, GuiCommand::Exit);
        let anchor = gui::Anchor {
            vertical: gui::VAnchor::Bottom,
            horizontal: gui::HAnchor::Right,
        };
        let direction = gui::Direction::Up;
        gui.add_layout(anchor, direction, vec![button_id_show_hide, button_id_exit]);

        Self { gui }
    }

    fn exit(&mut self, context: &mut Context) {
        context.add_command(hate::screen::Command::Pop);
    }

    fn handle_commands(&mut self, context: &mut Context) {
        while let Some(command) = self.gui.try_recv() {
            match command {
                // GuiCommand::A => println!("A"),
                // GuiCommand::B => println!("B"),
                // GuiCommand::RemoveButton => {
                //     let result = self.gui.remove(self.button_id_will_be_removed);
                //     println!("button_id_will_be_removed remove result: {:?}", result);
                //     let result2 = self.gui.remove(self.layout_id_a);
                //     println!("layout_id_a remove result: {:?}", result2);
                // }
                // GuiCommand::C => println!("C"),
                // GuiCommand::D => println!("D"),
                // GuiCommand::E => println!("E"),
                // GuiCommand::F => {
                //     println!("F");
                //     let new_sprite = gui::text_sprite(context, "FF", 0.1);
                //     self.gui
                //         .update_sprite(context, self.button_f_id, new_sprite);
                // }
                // GuiCommand::NextMap => self.select_next_map(context),
                GuiCommand::ShowHide => { /* TODO */ }
                GuiCommand::Exit => self.exit(context),
            }
        }
    }

    fn handle_event_click(&mut self, _: &mut Context, pos: Point) {
        self.gui.click(pos);
    }
}

impl Screen for ActionsTest {
    fn tick(&mut self, context: &mut Context, _: Time) {
        self.gui.draw(context);
    }

    fn handle_event(&mut self, context: &mut Context, event: Event) {
        match event {
            Event::Click { pos } => self.handle_event_click(context, pos),
            Event::Resize { aspect_ratio } => self.gui.resize(aspect_ratio),
        }
        self.handle_commands(context);
    }
}
