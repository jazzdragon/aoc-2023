use regex::Regex;

static INPUT_FILE: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl Round {
    fn from_str(s: &str) -> Self {
        let blue_re = Regex::new(r"(\d+) blue").unwrap();
        let red_re = Regex::new(r"(\d+) red").unwrap();
        let green_re = Regex::new(r"(\d+) green").unwrap();
        Round {
            blue: color_amount(blue_re, s),
            red: color_amount(red_re, s),
            green: color_amount(green_re, s),
        }
    }
}

impl Round {
    fn possible(&self) -> bool {
        self.blue < 15 && self.green < 14 && self.red < 13
    }
}

fn color_amount(re: Regex, s: &str) -> u32 {
    match re.captures(s) {
        Some(caps) => caps[1].parse().unwrap_or(0),
        None => 0,
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn possible(&self) -> bool {
        self.rounds.iter().all(|round| round.possible())
    }

    fn min_set_power(&self) -> u32 {
        let max_blue = self.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;
        let max_green = self.rounds.iter().max_by_key(|r| r.green).unwrap().green;
        let max_red = self.rounds.iter().max_by_key(|r| r.red).unwrap().red;
        max_blue * max_green * max_red
    }
}

impl Game {
    fn from_str(s: &str, id: u32) -> Self {
        let rounds = s
            .split_once(':')
            .unwrap()
            .1
            .split(';')
            .map(Round::from_str)
            .collect();
        Game { id, rounds }
    }
}

fn main() {
    let mut total = 0;
    let mut min_powers_total = 0;
    for (i, line) in INPUT_FILE.lines().enumerate() {
        let game = Game::from_str(line, u32::try_from(i + 1).unwrap());
        if game.possible() {
            total += game.id;
        }
        min_powers_total += game.min_set_power();
    }
    println!("Possible games: {}", total);
    println!("Min powers: {}", min_powers_total);
}
