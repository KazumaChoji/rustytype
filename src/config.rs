//! Configuration for RustyType.
//!
//! Designed for command-line arguments using [`clap`], but can be used
//! as a library too.


use clap::{ArgEnum, Parser};
use strum::EnumCount;
use termion::color;
use crate::wordlists::BuiltInWordlist;
use serde::{Serialize, Deserialize};
use toml;
const CLI_HELP: &str = "A trusty terminal typing tester.

Keyboard shortcuts:
ctrl-c: quit
ctrl-r: restart test with a new set of words
ctrl-w: delete last word
";

#[derive(Serialize, Deserialize)]
pub struct SavedSettings {
    num_words: usize,
    color_theme: ColorTheme,
    time_limit: Option<u64>,
    uppercase: bool,
    punctuation: bool,
}

impl Default for SavedSettings {
    fn default() -> Self {SavedSettings {
        num_words: 30,
        color_theme: ColorTheme::Normal,
        time_limit: Some(30),
        uppercase: false,
        punctuation: false,
        }
    }
}
impl SavedSettings {
    pub fn save (&self) -> Result<(), Box<dyn std::error::Error>>{
        let exe_path = std::env::current_exe()?;
        let toml_path = exe_path.parent().expect("how tf is this executing if theres no parent path").join("rustytype_settings.TOML");
        let toml_contents = toml::to_string(&self)?;
        std::fs::write(toml_path, toml_contents)?;
        Ok(())
    }
    
    pub fn load () -> Result<Self, Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let toml_path = exe_path.parent().expect("how tf is this executing if theres no parent path").join("rustytype_settings.TOML");
        let toml_contents = match std::fs::read_to_string(toml_path) {
            Ok(contents) => contents,
            Err(_) => return Ok(Self::default()),
        };
        Ok(toml::from_str(&toml_contents)?)
    }
}

#[derive(Clone, Copy, PartialEq, strum_macros::EnumCount, strum_macros::Display, Serialize, Deserialize)]
pub enum ColorTheme {
    Normal,
    MidnightBlue,
    Catpuccin,
}
impl Default for ColorTheme {
  fn default() -> Self {
      ColorTheme::Normal
  }
}

impl ColorTheme {
    fn to_index(&self) -> usize {
        match self {
            Self::Normal => 0,
            Self::MidnightBlue => 1,
            Self::Catpuccin => 2,
        }
    }

    fn from_index(i:usize) -> Self {
        match i {
            0 => Self::Normal,
            1 => Self::MidnightBlue,
            2 => Self::Catpuccin,
            _ => Self::Normal,
        }
    }

    pub fn next(&self) -> Self {
       ColorTheme::from_index((self.to_index() + 1) % ColorTheme::COUNT)
    }

    pub fn prev(&self) -> Self {
        // to prevent integer underflow, we loop around by adding total # of ColorTheme 
        ColorTheme::from_index((self.to_index()+ ColorTheme::COUNT-1 )% ColorTheme::COUNT)
    }
    
    pub fn correct_color(&self) -> color::Rgb {
       match self {
           Self::Normal => color::Rgb(144, 238, 144),      // light green
           Self::MidnightBlue => color::Rgb(100, 149, 237), // cornflower blue
           Self::Catpuccin => color::Rgb(166, 227, 161),    // catpuccin green
       } 
    }

    pub fn incorrect_color(&self) -> color::Rgb {
        match self {
            Self::Normal => color::Rgb(255, 0, 0),           // red
            Self::MidnightBlue => color::Rgb(255, 100, 100), // soft red
            Self::Catpuccin => color::Rgb(243, 139, 168),    // catpuccin red
        }

    }
    
    pub fn bg_color(&self) -> color::Rgb {
        match self {
            Self::Normal => color::Rgb(40, 40, 40),
            Self::MidnightBlue => color::Rgb(25, 25, 112),
            Self::Catpuccin => color::Rgb(30, 30, 46),
        }
    }
}

/// Main configuration for RustyType.
#[derive(Parser)]
#[clap(author, version, about = CLI_HELP)]
pub struct CliArgs {
    /// Word list name.
    #[clap(arg_enum, short, long, default_value_t = BuiltInWordlist::Top250)]
    pub wordlist: BuiltInWordlist,
    /// Path to custom word list file.
    ///
    /// This argument cannot be used along with `-w`/`--wordlist`
    #[clap(short = 'f', long = "file", conflicts_with = "wordlist")]
    pub wordlist_file: Option<String>,
    /// Number of words to show on each test.
    #[clap(short, long)]
    pub num_words: Option<usize>,
    /// Whether to include punctuation
    #[clap(short, long)]
    pub punctuation: Option<bool>,
    /// Whether to show words in UPPERCASE
    #[clap(short, long)]
    pub uppercase: Option<bool>,
    /// Whether to add a time limit in seconds
    #[clap(short, long)]
    pub time_limit: Option<u64>,
    #[clap(skip)] 
    pub color_theme: ColorTheme,
}

pub struct RustyTypeConfig {
    pub wordlist: BuiltInWordlist,
    pub wordlist_file: Option<String>,
    pub num_words: usize,
    pub punctuation: bool,
    pub uppercase: bool, 
    pub time_limit: Option<u64>,
    pub color_theme: ColorTheme,
}
impl RustyTypeConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cli = CliArgs::parse();
        let saved = SavedSettings::load()?;
        Ok(Self {
            num_words: cli.num_words.unwrap_or(saved.num_words),
            uppercase: cli.uppercase.unwrap_or(saved.uppercase),
            punctuation: cli.punctuation.unwrap_or(saved.punctuation),
            time_limit: cli.time_limit.or(saved.time_limit),
            color_theme: saved.color_theme,
            wordlist_file: cli.wordlist_file,
            wordlist: cli.wordlist,
        })
    }

    /// Name of the text used for typing test
    pub fn text_name(&self) -> String {
        if let Some(wordlist_file) = &self.wordlist_file {
            format!("custom file `{}`", wordlist_file)
        } else {
            if let Some(possible_value) = self.wordlist.to_possible_value() {
                possible_value.get_name()
            } else {
                "unknown"
            }
            .to_string()
        }
    }


}

// this is how you convert from one type to another idiomatically i guess
impl From<&RustyTypeConfig> for SavedSettings {
    fn from(config: &RustyTypeConfig) -> Self {
        SavedSettings {
            num_words: config.num_words,
            color_theme: config.color_theme,
            time_limit: config.time_limit,
            punctuation: config.punctuation,
            uppercase: config.uppercase, 
        }
    }
}

