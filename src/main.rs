use iced::{
    alignment::Alignment,
    widget::{
        column, container, horizontal_rule, row, text, text_editor,
        text_editor::{Action, Edit},
    },
    Element, Fill, Length, Pixels, Task, Theme,
};
use std::borrow::Cow;

mod textstat;
use textstat::{calculate_stats, TextStats};

fn main() -> iced::Result {
    let theme = |_s: &WordCounter| Theme::Dark;

    iced::application("QuickCount", WordCounter::update, WordCounter::view)
        .theme(theme)
        .centered()
        .run()
}

#[derive(Default)]
struct WordCounter {
    content: text_editor::Content,
    stats: TextStats,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    TextModified,
    NoOp,
}

impl WordCounter {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                content: iced::widget::text_editor::Content::<_>::with_text(""),
                stats: TextStats::default(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Word Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(text) => {
                self.content = iced::widget::text_editor::Content::<_>::with_text(&text);
                self.stats = calculate_stats(&text);
            }
            Message::TextModified => {
                // Handle modifications here if needed
            }
            Message::NoOp => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let determine_action = |action| match action {
            Edit::Insert(c) => Message::TextInputChanged(c.to_string()),
            Edit::Paste(text) => Message::TextInputChanged((*text).clone()),
            Edit::Backspace | Edit::Delete => Message::TextModified,
            _ => Message::NoOp,
        };

        let text_area = text_editor(&self.content)
            .on_action(move |action| match action {
                Action::Edit(edit_action) => determine_action(edit_action),
                _ => Message::NoOp,
            })
            .padding(10)
            .width(Pixels(600.0))
            .height(Length::Fixed(150.0));

        let stats = &self.stats;
        let stats_view = column![
            stat_row("Words:", format!("{}", stats.word_count)),
            stat_row("Characters (no spaces:", format!("{}", stats.letter_count)),
            stat_row("Sentences:", format!("{}", stats.sentence_count)),
            stat_row("Paragraphs:", format!("{}", stats.paragraph_count)),
            stat_row(
                "Avg word length:",
                format!("{:.2}", stats.average_word_length)
            ),
            stat_row(
                "Avg sentence length:",
                format!("{:.2}", stats.average_sentence_length)
            ),
            stat_row("Longest word:", &stats.longest_word),
            stat_row("Most common word:", &stats.most_common_word),
            stat_row("Unique words:", format!("{}", stats.unique_word_count)),
            stat_row("Readability Level:", &stats.english_level),
            stat_row(
                "Flesch-Kincaid:",
                format!("{:.2}", stats.flesch_kincaid_grade)
            ),
            stat_row(
                "Gunning Fog:",
                format!(
                    "{:.2} - {}",
                    stats.gunning_fog_index, stats.fog_interpretation
                )
            ),
            stat_row(
                "SMOG Grade:",
                format!("{:.2} - {}", stats.smog_grade, stats.smog_interpretation)
            ),
        ]
        .spacing(5);

        let description = text(
            "This application is a simple yet powerful text analysis tool. It provides various statistics \
            about your text including readability scores. Contributions are welcome!"
        )
        .size(14)
        .width(600);

        let footer = row![text("Created with Iced"), text(" - "), text("Main Page")]
            .spacing(5)
            .align_y(Alignment::Center);

        let content = column![
            text("Word Counter").size(30),
            text_area,
            stats_view,
            description,
            horizontal_rule(1),
            footer,
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Fill)
            .into()
    }
}

fn stat_row<'a>(
    label: impl Into<Cow<'a, str>> + iced::widget::text::IntoFragment<'a>,
    value: impl Into<Cow<'a, str>> + iced::widget::text::IntoFragment<'a>,
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(200.0)).size(14),
        text(value).size(14),
    ]
    .spacing(10)
    .into()
}
