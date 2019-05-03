
use crate::{
    part_traits::{
        config_traits::{
            ConfigHandler,
            ConfigParser,
            ConfigHandlerError
        },
        action_traits::{
            ActionHandler,
            ActionParser
        },
        board_traits::{
            BoardT
        }
    },
    Message,
    Number,
    UserInput
};
use super::{
    board::MineBoard,
    config::Settings,
    action::UserAction
};

pub struct Minesweeper {
    game_board: MineBoard,
    game_settings: Settings
}

impl Minesweeper {
    pub fn new(init: Settings) -> Self {
        Minesweeper {
            game_board: MineBoard::new(init),
            game_settings: init
        }
    }
}

impl ActionHandler for Minesweeper {
    type ActionThing = UserAction;
    type ResultThing = Result<Message,Message>;

    fn do_action(&mut self, _action: Self::ActionThing) -> Self::ResultThing {
        Ok("".to_string())
    }
}

impl ActionParser for Minesweeper {
    type ActionThing = UserAction;
    type UserInputThing = UserInput;

    fn parse(_input: Self::UserInputThing) -> Self::ActionThing {
        Self::ActionThing::default()
    }

    fn try_parse(_input: Self::UserInputThing) -> Option<Self::ActionThing> {
        Some(Self::ActionThing::default())
    }
}

impl ConfigHandler<Number,Number> for Minesweeper {
    type Key = Number;
    type Value = Number;
    type ConfigThing = Settings;
    type UserInputThing = UserInput;
    type MessageThing = Message;

    fn try_get(_config: &Self::ConfigThing, _ukey: Self::UserInputThing) -> Result<&Number, ConfigHandlerError> {
        Ok(&0)
    }

    fn try_set(_config: &mut Self::ConfigThing, _key: Self::UserInputThing, val: Self::UserInputThing) -> Result<&Number,ConfigHandlerError> {
        Ok(&0)
    }
}

impl ConfigParser<Number> for UserInput {
    fn parse(s: Self) -> Number {
        0
    }

    fn try_parse(s: Self) -> Option<Number> {
        Some(0)
    }
}