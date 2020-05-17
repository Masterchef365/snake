mod board;
mod game;
mod snake_widget;
mod ai;
use ai::input_neurons;
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
    Event(iced_native::Event),
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
                        StepResult::Died => {
                            self.paused = true;
                            println!("Died! Score: {}", self.game.score());
                        }
                    }
                    println!("{:?}", input_neurons(&self.game));
                }
            }
            Message::Event(iced_native::Event::Keyboard(
                    iced_native::keyboard::Event::KeyPressed { key_code, .. },
            )) => {
                use iced_native::keyboard::KeyCode;
                match key_code {
                    KeyCode::Up => self.game.set_direction(Direction::Up),
                    KeyCode::Down => self.game.set_direction(Direction::Down),
                    KeyCode::Right => self.game.set_direction(Direction::Right),
                    KeyCode::Left => self.game.set_direction(Direction::Left),
                    KeyCode::Space => return async { Message::Tick }.into(),
                    _ => (),
                }
            }
            _ => (),
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        //Subscription::batch(vec![
            iced_native::subscription::events().map(Message::Event)//,
            //time::every(std::time::Duration::from_millis(10000)).map(|_| Message::Tick),
        //])
    }

    fn view(&mut self) -> Element<Self::Message> {
        let widget = SnakeWidget::new(self.game.board(), 20.0, 1.0);
        Canvas::new(widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
