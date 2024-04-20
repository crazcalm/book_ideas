use std::fs::File;

use rand::seq::SliceRandom;

struct Card {
    front: String,
    back: String,
}

impl Card {
    fn new(front: String, back: String) -> Self {
        Self { front, back }
    }

    fn front(&self) -> &str {
        self.front.as_str()
    }

    fn back(&self) -> &str {
        self.back.as_str()
    }
}

fn ui(state: &str, text: &str, count: usize, total: usize) -> String {
    format!(
        "{state}: {text}

{count}/{total}

(p)revious (n)ext (f)lip (s)huffle (q)uit

"
    )
}

#[derive(Debug, PartialEq)]
enum Command {
    Previous,
    Next,
    Flip,
    Quit,
    Shuffle,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    match input.to_lowercase().as_str() {
        "n" | "next" => Command::Next,
        "p" | "previous" => Command::Previous,
        "f" | "flip" => Command::Flip,
        "q" | "quit" => Command::Quit,
        "s" | "shuffle" => Command::Shuffle,
        _ => Command::Unknown,
    }
}

fn load_from_csv(reader: impl std::io::Read) -> Result<Vec<Card>, std::io::Error> {
    let mut rdr = csv::Reader::from_reader(reader);

    let mut cards = Vec::new();
    for result in rdr.records() {
        let record = result?;

        cards.push(Card::new(
            record[0].to_string().trim().to_string(),
            record[1].to_string().trim().to_string(),
        ));
    }

    Ok(cards)
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);

    let file_path = args.next().expect("Must pass the path to the csv file");

    let file_reader = File::open(file_path).expect("Unable to open file");

    let mut cards = load_from_csv(file_reader).expect("Unable to load csv file");

    // Clearing the screen
    let _ = clearscreen::clear();

    // Set initial state for application
    let mut command = Command::Unknown;
    let mut current_index = 0;
    let mut state = "front";
    let mut text = cards[current_index].front();

    while command != Command::Quit {
        println!("{}", ui(state, text, current_index + 1, cards.len()));
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        command = parse_command(user_input.trim());

        match command {
            Command::Next => {
                if current_index + 1 < cards.len() {
                    current_index += 1;
                    state = "front";
                    text = cards[current_index].front();
                }
            }
            Command::Previous => {
                if current_index > 0 {
                    current_index -= 1;
                    state = "front";
                    text = cards[current_index].front();
                }
            }
            Command::Flip => {
                if state.eq("back") {
                    state = "front";
                    text = cards[current_index].front();
                } else {
                    state = "back";
                    text = cards[current_index].back();
                }
            }
            Command::Shuffle => {
                let mut rng = rand::thread_rng();
                cards.shuffle(&mut rng);
                current_index = 0;
                state = "front";
                text = cards[current_index].front();
            }
            Command::Quit | Command::Unknown => {}
        }

        // Clearing the screen
        let _ = clearscreen::clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_methods() {
        let front = "Front";
        let back = "Back";

        let card = Card::new(front.to_string(), back.to_string());

        assert_eq!(front, card.front());
        assert_eq!(back, card.back());
    }

    #[test]
    fn test_ui() {
        let expected_ui = "front: front of card

1/10

(p)revious (n)ext (f)lip (s)huffle (q)uit

";

        let card_ui = ui("front", "front of card", 1, 10);

        assert_eq!(card_ui, expected_ui);
    }

    #[test]
    fn test_parse_command() {
        let cases = vec![
            ("n", Command::Next),
            ("next", Command::Next),
            ("p", Command::Previous),
            ("previous", Command::Previous),
            ("f", Command::Flip),
            ("flip", Command::Flip),
            ("q", Command::Quit),
            ("quit", Command::Quit),
            ("s", Command::Shuffle),
            ("shuffle", Command::Shuffle),
            ("unknown", Command::Unknown),
        ];

        for (index, (input, expected)) in cases.into_iter().enumerate() {
            assert_eq!(parse_command(input), expected, "Case {index} failed");
        }
    }

    #[test]
    fn test_load_from_csv() {
        let data = "\
front, back,
front_1, back_1,
front_2, back_2,
front_3, back_3,
";

        let cards = load_from_csv(data.as_bytes()).unwrap();

        assert_eq!(cards[0].front(), "front_1");
        assert_eq!(cards[0].back(), "back_1");
        assert_eq!(cards[1].front(), "front_2");
        assert_eq!(cards[1].back(), "back_2");
        assert_eq!(cards[2].front(), "front_3");
        assert_eq!(cards[2].back(), "back_3");
    }
}
