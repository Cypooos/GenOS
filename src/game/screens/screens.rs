use alloc::{boxed::Box, string::ToString, vec};

use crate::game::screens::pop_up::PopUp;

use super::{Screenable, SA};

use super::choke_talk::{ChokeFace, RpgDial};
use super::level::{Level, LevelChoice};
use super::menus::{OneScreenMenu, PasswordMenu};
// use super::visual::GifVeiwer;

pub struct GameLogic {
    pub restart_lvl1: usize, // nombre de fois qu'il a restart le level 1
    pub got_glitched: bool,
    pub nb_errors: usize, // nombre d'erreurs
}

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    MainMenu,
    CreditMenu,

    FilesPassword,
    Files,

    Intro(usize),

    Level1,
}

pub fn screen_to_instance(ele: Screen) -> Box<dyn Screenable> {
    match ele {
        Screen::MainMenu => Box::new(OneScreenMenu::MainMenu),
        Screen::CreditMenu => Box::new(PopUp::new(
            "Credits",
            (20, 10),
            (10, 10),
            vec![
                "".to_string(),
                "".to_string(),
                "This game is cool".to_string(),
            ],
            vec![SA::Restore, SA::Load(Screen::MainMenu)],
            "$30",
        )),
        Screen::FilesPassword => Box::new(PasswordMenu::new(
            "456789",
            vec![SA::Restore, SA::Load(Screen::Files)],
            vec![SA::Restore],
        )),

        Screen::Intro(x) => match x {
            0 => Box::new(RpgDial::new(
                ChokeFace::Normal,
                "$0E - Choke -",
                ["", "", "Hi player. Welcome to this game.", "", ""],
                vec![SA::Change(Screen::Intro(1))],
            )),
            1 => Box::new(RpgDial::new(
                ChokeFace::Normal,
                "$0E - Choke -",
                [
                    "",
                    "",
                    "To be honest, it isn't a good game.",
                    "",
                    "Like not at all. You will hate it.",
                ],
                vec![SA::Change(Screen::Intro(2))],
            )),
            2 => Box::new(RpgDial::new(
                ChokeFace::Happy,
                "$0E - Choke -",
                [
                    "",
                    "But I will have fun, finally !",
                    "",
                    "After all thoses years...",
                    "",
                ],
                vec![SA::Change(Screen::Intro(3))],
            )),
            3 => Box::new(RpgDial::new(
                ChokeFace::Normal,
                "$0E - Choke -",
                [
                    "",
                    "Yep, let's get to buisness you dummy.",
                    "",
                    "( not calling names, but if you failed this you are",
                    "  kinda dumb. )",
                ],
                vec![SA::Restore, SA::Change(Screen::Level1)],
            )),
            _ => Box::new(OneScreenMenu::_404),
        },
        Screen::Level1 => Box::new(Level::new(
            "Level 1",
            ("", "The good choice is the first", ""),
            vec![
                LevelChoice::new("Choice 1", "I am the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 2", "I am NOT the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 3", "I am NOT the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 4", "I am NOT the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 5", "I am NOT the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 6", "I am NOT the correct choice", Screen::MainMenu),
                LevelChoice::new("Choice 7", "I am NOT the correct choice", Screen::MainMenu),
            ],
        )),
        _ => Box::new(OneScreenMenu::_404),
    }
}
