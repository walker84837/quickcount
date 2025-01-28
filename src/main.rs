use iced::{
    alignment, color,
    widget::{column, container, horizontal_rule, row, text},
    Element, Font, Length, Settings, Task, Theme,
};
use regex::Regex;
use std::collections::HashMap;

fn main() -> iced::Result {
    WordCounter::run(Settings::default())
}

struct WordCounter {
    text_input: String,
    stats: TextStats,
}

#[derive(Debug, Default)]
struct TextStats {
    word_count: usize,
    letter_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
    average_word_length: f64,
    average_sentence_length: f64,
    longest_word: String,
    most_common_word: String,
    unique_word_count: usize,
    flesch_kincaid_grade: f64,
    gunning_fog_index: f64,
    smog_grade: f64,
    english_level: String,
    smog_interpretation: String,
    fog_interpretation: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl WordCounter {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                text_input: String::new(),
                stats: TextStats::default(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Word Counter")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextInputChanged(text) => {
                self.text_input = text;
                self.stats = calculate_stats(&self.text_input);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let text_area = text_area("", &self.text_input)
            .on_input(Message::TextInputChanged)
            .padding(10)
            .width(Length::Units(600))
            .height(Length::Units(150));

        let stats = &self.stats;
        let stats_view = column![
            stat_row("Words:", &format!("{}", stats.word_count)),
            stat_row(
                "Characters (no spaces):",
                &format!("{}", stats.letter_count)
            ),
            stat_row("Sentences:", &format!("{}", stats.sentence_count)),
            stat_row("Paragraphs:", &format!("{}", stats.paragraph_count)),
            stat_row(
                "Avg word length:",
                &format!("{:.2}", stats.average_word_length)
            ),
            stat_row(
                "Avg sentence length:",
                &format!("{:.2}", stats.average_sentence_length)
            ),
            stat_row("Longest word:", &stats.longest_word),
            stat_row("Most common word:", &stats.most_common_word),
            stat_row("Unique words:", &format!("{}", stats.unique_word_count)),
            stat_row("Readability Level:", &stats.english_level),
            stat_row(
                "Flesch-Kincaid:",
                &format!("{:.2}", stats.flesch_kincaid_grade)
            ),
            stat_row(
                "Gunning Fog:",
                &format!(
                    "{:.2} - {}",
                    stats.gunning_fog_index, stats.fog_interpretation
                )
            ),
            stat_row(
                "SMOG Grade:",
                &format!("{:.2} - {}", stats.smog_grade, stats.smog_interpretation)
            ),
        ]
        .spacing(5);

        let description = text(
            "This application is a simple yet powerful text analysis tool. It provides various statistics \
            about your text including readability scores. Contributions are welcome!"
        )
        .size(14)
        .width(600);

        let footer = row![
            text("Created with ").size(14),
            text("Iced").style(color!(0x8BDFFA)),
            text(" - "),
            text("Main Page").style(color!(0xc17c71))
        ]
        .spacing(5)
        .align_items(alignment::Alignment::Center);

        let content = column![
            text("Word Counter").size(30).style(color!(0xebd1c6)),
            text_area,
            stats_view,
            description,
            horizontal_rule(1),
            footer,
        ]
        .spacing(20)
        .align_items(alignment::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(container::Appearance {
                background: Some(color!(0x4b281b).into()),
                ..Default::default()
            })
            .into()
    }
}

fn stat_row<'a>(label: &str, value: &str) -> Element<'a, Message> {
    row![
        text(label).width(Length::Units(200)).size(14),
        text(value).size(14),
    ]
    .spacing(10)
    .into()
}

fn calculate_stats(text: &str) -> TextStats {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    let letter_count = text.chars().filter(|c| !c.is_whitespace()).count();

    let sentence_re = Regex::new(r"[.!?]").unwrap();
    let sentence_count = sentence_re.find_iter(text).count();

    let paragraph_count = text.split("\n\n").filter(|p| !p.trim().is_empty()).count();

    let total_word_length: usize = words.iter().map(|w| w.len()).sum();
    let average_word_length = if word_count > 0 {
        total_word_length as f64 / word_count as f64
    } else {
        0.0
    };

    let average_sentence_length = if sentence_count > 0 {
        word_count as f64 / sentence_count as f64
    } else {
        0.0
    };

    let longest_word = words
        .iter()
        .max_by_key(|w| w.len())
        .unwrap_or(&"")
        .to_string();

    let mut word_counts = HashMap::new();
    for word in words {
        *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    let (most_common_word, _) = word_counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .unwrap_or((&String::new(), &0));

    let unique_word_count = word_counts.len();

    // Readability calculations
    let total_syllables: usize = words.iter().map(|w| count_syllables(w)).sum();
    let complex_word_count = words.iter().filter(|w| count_syllables(w) >= 3).count();

    let flesch_kincaid_grade = 0.39 * average_sentence_length
        + 11.8 * (total_syllables as f64 / word_count as f64)
        - 15.59;

    let gunning_fog_index =
        0.4 * (average_sentence_length + 100.0 * (complex_word_count as f64 / word_count as f64));

    let smog_grade =
        1.043 * (complex_word_count as f64 * (30.0 / sentence_count as f64)).sqrt() + 3.1291;

    let english_level = if flesch_kincaid_grade <= 5.0 {
        "Basic"
    } else if flesch_kincaid_grade <= 8.0 {
        "Intermediate"
    } else {
        "Advanced"
    }
    .to_string();

    let (smog_interpretation, fog_interpretation) =
        interpret_readability(smog_grade, gunning_fog_index);

    TextStats {
        word_count,
        letter_count,
        sentence_count,
        paragraph_count,
        average_word_length,
        average_sentence_length,
        longest_word,
        most_common_word: most_common_word.to_string(),
        unique_word_count,
        flesch_kincaid_grade,
        gunning_fog_index,
        smog_grade,
        english_level,
        smog_interpretation,
        fog_interpretation,
    }
}

fn count_syllables(word: &str) -> usize {
    let word = word.to_lowercase();
    let vowels = Regex::new(r"[aeiouy]+").unwrap();
    let diphthong = Regex::new(r"[aeiou]{2}").unwrap();
    let tripthong = Regex::new(r"[aeiou]{3}").unwrap();
    let leading_trailing = Regex::new(r"^[^aeiouy]+|[^aeiouy]+$").unwrap();

    let cleaned = leading_trailing.replace_all(&word, "");
    let cleaned = tripthong.replace_all(&cleaned, "a");
    let cleaned = diphthong.replace_all(&cleaned, "a");

    vowels.find_iter(&cleaned).count().max(1)
}

fn interpret_readability(smog: f64, fog: f64) -> (String, String) {
    let smog_interp = match smog {
        x if x <= 6.0 => "Basic English",
        x if x <= 9.0 => "Intermediate English",
        x if x <= 12.0 => "Upper Intermediate",
        x if x <= 16.0 => "Advanced",
        _ => "Very Advanced",
    }
    .to_string();

    let fog_interp = match fog {
        x if x <= 8.0 => "Basic English",
        x if x <= 12.0 => "Intermediate",
        x if x <= 16.0 => "Advanced",
        _ => "Very Advanced",
    }
    .to_string();

    (smog_interp, fog_interp)
}
