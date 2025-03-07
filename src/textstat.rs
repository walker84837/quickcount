use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TextStats {
    pub word_count: usize,
    pub letter_count: usize,
    pub sentence_count: usize,
    pub paragraph_count: usize,
    pub average_word_length: f64,
    pub average_sentence_length: f64,
    pub longest_word: String,
    pub most_common_word: String,
    pub unique_word_count: usize,
    pub flesch_kincaid_grade: f64,
    pub gunning_fog_index: f64,
    pub smog_grade: f64,
    pub english_level: String,
    pub smog_interpretation: String,
    pub fog_interpretation: String,
}

pub fn calculate_stats(text: &str) -> TextStats {
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
    for word in &words {
        *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    let b = String::new();
    let (most_common_word, _) = word_counts
        .iter()
        .max_by_key(|&(_, &count)| count)
        .unwrap_or((&b, &0));

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

pub fn interpret_readability(smog: f64, fog: f64) -> (String, String) {
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
