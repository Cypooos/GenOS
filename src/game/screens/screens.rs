use alloc::{boxed::Box, string::ToString, vec};

use super::Screenable;

use super::choke_talk::{ChokeFace, SimpleDialogue};
use super::level::{Level, LevelChoice};
use super::menus::{OneScreenMenu, PasswordMenu};
use super::visual::GifVeiwer;

// ONERR: Bien vérifie que l'enum correspond au vec !!!!

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    MainMenu = 0,
    CreditMenu,
    DebugPasswordMenu,

    TestMenu,
    TestDialogueNormal,
    TestDialogueHappy,
    TestDialogueOwO,
    TestGif,
    TestLevel3Simple,
}

pub fn screen_to_instance(ele: Screen) -> Box<dyn Screenable> {
    match ele {
        Screen::MainMenu => Box::new(OneScreenMenu::MainMenu),
        Screen::CreditMenu => Box::new(OneScreenMenu::CreditMenu),
        Screen::DebugPasswordMenu => Box::new(PasswordMenu::new(
            "123456",
            Screen::TestDialogueNormal,
            Screen::MainMenu,
        )),

        Screen::TestMenu => Box::new(OneScreenMenu::TestMenu),

        Screen::TestDialogueNormal => Box::new(SimpleDialogue::new(
            ChokeFace::Normal,
            [
                "Hellowo !".to_string(),
                "".to_string(),
                "This is a very simple".to_string(),
                "test text".to_string(),
                "hope u liked it !".to_string(),
            ],
            Screen::TestDialogueHappy,
        )),
        Screen::TestDialogueHappy => Box::new(SimpleDialogue::new(
            ChokeFace::Happy,
            [
                "Hey, I am now happy !".to_string(),
                "Hey, I am now happy !".to_string(),
                "Hey, I am now happy !".to_string(),
                "Hey, I am now happy !".to_string(),
                "Hey, I am now happy !".to_string(),
            ],
            Screen::TestDialogueOwO,
        )),
        // OwO
        Screen::TestDialogueOwO => Box::new(SimpleDialogue::new(
            ChokeFace::OwO,
            [
                "What.".to_string(),
                "The.".to_string(),
                "Fuck.".to_string(),
                "".to_string(),
                "      me = OwO".to_string(),
            ],
            Screen::TestGif,
        )),
        Screen::TestGif => Box::new(GifVeiwer::new(
            vec![
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | __ __ |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              |() ()  |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              |  () ()|
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | () () |
              (   ^   )
               \|,,,|/
                |"""| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | {} {} |
              (   ^   )
               \|---|/
                |---| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | {} {} |
              (   ^   )
               \|---|/
                |___| 
              "#
                .to_string(),
                r#"
                _____  
               /     \ 
              | {} {} |
              (   ^   )
               \|,,,|/
                |---| 
              "#
                .to_string(),
            ],
            4,
            Screen::MainMenu,
        )),
        Screen::TestLevel3Simple => Box::new(Level::new(
            "A simple test level",
            ("", "A cool level no ?", "third is gud"),
            vec![
                LevelChoice::new("Choice 1", "not Pog", Screen::MainMenu),
                LevelChoice::new("Choice 2", "not Pog", Screen::MainMenu),
                LevelChoice::new("Choice 3", "veryyyy Pog", Screen::MainMenu),
            ],
            Screen::MainMenu,
            Screen::TestMenu,
        )),
    }
}
