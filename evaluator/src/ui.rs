use snake_game::game::*;
use iced::canvas::*;
use iced::*;
use crate::snake_widget::SnakeWidget;
use snake_trainer::neuralnet::NeuralNet;

pub struct SnakeApp {
    game: Game,
    paused: bool,
    net: NeuralNet,
}

#[derive(Debug)]
pub enum Message {
    Tick,
    Event(iced_native::Event),
}

impl SnakeApp {
    fn new_game(&mut self) {
        self.paused = false;
        self.game = Game::new(self.game.width, self.game.height);
        println!("NEW GAME");
    }
}

impl Application for SnakeApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = (NeuralNet, usize, usize);

    fn new((net, width, height): (NeuralNet, usize, usize)) -> (Self, Command<Self::Message>) {
        (
            Self {
                game: Game::new(width, height),
                net,
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
                    self.net.play(&mut self.game);
                    match self.game.step() {
                        StepResult::Alive => self.paused = false,
                        StepResult::Died => {
                            println!("Died! Score: {}", self.game.score());
                            self.new_game();
                        }
                    }
                }
            }
            Message::Event(iced_native::Event::Keyboard(
                    iced_native::keyboard::Event::KeyPressed { key_code, .. },
            )) => {
                use iced_native::keyboard::KeyCode;
                match key_code {
                    /*
                    KeyCode::Up => self.game.set_direction(Direction::Up),
                    KeyCode::Down => self.game.set_direction(Direction::Down),
                    KeyCode::Right => self.game.set_direction(Direction::Right),
                    KeyCode::Left => self.game.set_direction(Direction::Left),
                    */
                    KeyCode::R => self.new_game(),
                    KeyCode::T => return async { Message::Tick }.into(),
                    KeyCode::Space => self.paused = !self.paused,
                    _ => (),
                }
            }
            _ => (),
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::Event),
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
        ])
    }

    fn view(&mut self) -> Element<Self::Message> {
        let widget = SnakeWidget::new(self.game.board(), 20.0, 1.0);
        Canvas::new(widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

