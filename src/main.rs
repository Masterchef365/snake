mod board;
mod game;
mod snake_widget;
use game::*;
use iced::canvas::*;
use iced::*;
use snake_widget::SnakeWidget;

fn main() {
    <SnakeApp as Application>::run(Default::default());
}

struct SnakeApp {
    game: Game,
    paused: bool,
}

#[derive(Debug)]
pub enum Message {
    Tick,
}

impl Application for SnakeApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                game: Game::new(15, 15),
                paused: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Snake evolution")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                if !self.paused {
                    match self.game.step() {
                        StepResult::Alive => self.paused = false,
                        StepResult::Died => self.paused = true,
                    }
                }
            },
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let widget = SnakeWidget::new(self.game.board(), 20.0, 1.0);
        Canvas::new(widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
