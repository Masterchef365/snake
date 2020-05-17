mod snake_widget;
mod ui;
use snake_trainer::neuralnet::NeuralNet;
use ui::SnakeApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1).peekable();

    if args.peek().is_none() {
        println!("Usage: net_path width height");
        return Ok(());
    }

    let net_path: String = args.next().unwrap();
    let width: usize = args.next().unwrap().parse()?;
    let height: usize = args.next().unwrap().parse()?;

    let model = NeuralNet::load(net_path)?;

    <SnakeApp as iced::Application>::run(iced::Settings::with_flags((model, width, height)));

    Ok(())
}
