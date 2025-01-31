// Task may be needed
use iced::{
    alignment::Alignment,
    widget::{
        column, container, horizontal_rule, row, text, text_editor,
        text_editor::{Action, Edit},
    },
    Element, Fill, Length, Pixels, Theme,
};
use std::borrow::Cow;

mod qc_editor;
mod textstat;
use crate::{
    qc_editor::QCEditor,
    textstat::{calculate_stats, TextStats},
};

const NAME: &str = "QuickCount";

fn main() -> iced::Result {
    let theme = |_s: &QuickCount| Theme::Dark;

    iced::application(NAME, QuickCount::update, QuickCount::view)
        .theme(theme)
        .centered()
        .run()
}

#[derive(Default)]
struct QuickCount {
    content: text_editor::Content,
    stats: TextStats,
    editor: QCEditor,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    TextDeleted(DeleteState),
    NoOp,
}

#[derive(Debug, Clone)]
enum DeleteType {
    BeforeCursor,
    AfterCursor,
}

type DeleteState = (DeleteType, (usize, usize));

impl QuickCount {
    fn update(&mut self, message: Message) {
        match message {
            Message::TextInputChanged(text) => {
                // Append the new text and update our models
                self.editor.add_new_content(text.clone());
                self.content =
                    iced::widget::text_editor::Content::<_>::with_text(&self.editor.content);
                self.stats = calculate_stats(&self.editor.content);
            }
            Message::TextDeleted(delete_state) => {
                self.handle_text_deletion(delete_state);
            }
            Message::NoOp => {}
        }
    }

    /// Helper function: given the current text, a target line and column,
    /// return the corresponding byte index.
    fn pos_to_index(text: &str, target_line: usize, target_col: usize) -> Option<usize> {
        let mut current_line = 0;
        let mut current_col = 0;
        for (i, ch) in text.char_indices() {
            if current_line == target_line && current_col == target_col {
                return Some(i);
            }
            if ch == '\n' {
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
        }
        // If we are at the end of the text and the position matches,
        // return text.len()
        if current_line == target_line && current_col == target_col {
            Some(text.len())
        } else {
            None
        }
    }

    /// Handles deletion by converting the (line, column) position into a byte
    /// index and then removing the proper character.
    fn handle_text_deletion(&mut self, delete_state: DeleteState) {
        let (delete_type, (line, column)) = delete_state;
        if let Some(index) = Self::pos_to_index(&self.editor.content, line, column) {
            match delete_type {
                DeleteType::BeforeCursor => {
                    if index > 0 {
                        // Remove the character immediately before the cursor.
                        // Find the start of the previous character.
                        let char_start = self.editor.content[..index]
                            .char_indices()
                            .rev()
                            .next()
                            .map(|(i, _)| i)
                            .unwrap();
                        self.editor.content.replace_range(char_start..index, "");
                    }
                }
                DeleteType::AfterCursor => {
                    if index < self.editor.content.len() {
                        // Remove the character at the cursor.
                        let char_end = self.editor.content[index..]
                            .char_indices()
                            .next()
                            .map(|(i, ch)| index + i + ch.len_utf8())
                            .unwrap();
                        self.editor.content.replace_range(index..char_end, "");
                    }
                }
            }
            // Update the content and the text statistics.
            self.content = iced::widget::text_editor::Content::with_text(&self.editor.content);
            self.stats = calculate_stats(&self.editor.content);
        }
    }

    fn determine_action(&self, action: Action) -> Message {
        let pos = self.content.cursor_position();
        match action {
            Action::Edit(edit_action) => match edit_action {
                Edit::Insert(c) => Message::TextInputChanged(c.to_string()),
                Edit::Paste(text) => Message::TextInputChanged((*text).clone()),
                Edit::Backspace => Message::TextDeleted((DeleteType::BeforeCursor, pos)),
                Edit::Delete => Message::TextDeleted((DeleteType::AfterCursor, pos)),
                _ => Message::NoOp,
            },
            _ => Message::NoOp,
        }
    }

    fn view(&self) -> Element<Message> {
        let text_area = text_editor(&self.content)
            .on_action(move |action| match action {
                Action::Edit(edit_action) => {
                    Self::determine_action(&self, Action::Edit(edit_action))
                }
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

        let footer = row![
            text("Made with <3 by walker84837"),
            text(" - "),
            text("Feel free to contribute at https://github.com/walker84837/quickcount")
        ]
        .spacing(5)
        .align_y(Alignment::Center);

        let content = column![
            text(NAME).size(30),
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
