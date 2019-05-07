
use crate::{
    part_traits::{
        config_traits::{
            ConfigT,
            ConfigHandler,
            ConfigParser,
            ConfigHandlerError
        },
        action_traits::{
            ActionT,
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
    action::UserAction,
    actiontype::UserActionType
};

pub struct Minesweeper {
    game_board: MineBoard,
    game_settings: Settings,
    game_over: bool
}

impl Minesweeper {
    pub fn new(init: Settings) -> Self {
        Minesweeper {
            game_board: MineBoard::new(),
            game_settings: init,
            game_over: false
        }
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn start(&mut self) -> Result<&'static str,Message> {
        let width = self.game_settings.get(0);
        let height = self.game_settings.get(1);
        let bombs = self.game_settings.get(3);
        match self.game_board.init(width, height, bombs) {
            Ok(_) => Ok("Started game."),
            Err(e) => Err(e)
        }
    }

    pub fn show_board(&self) {
        print!("{}", self.game_board);
    }

    pub fn handle_input(&mut self, input: UserInput) -> Result<Message,&'static str> {
        self.do_action(Minesweeper::parse(input))
    }
}

impl ActionHandler for Minesweeper {
    type ActionThing = UserAction;
    type ResultThing = Result<Message,&'static str>;

    fn do_action(&mut self, action: Self::ActionThing) -> Self::ResultThing {
        use UserActionType::*;
        match action.get_type() {
            Unknown => return Err("Unknown action."),
            Reset => return Ok(self.start().unwrap().to_string()),
            Set => (),
            Mark => (),
            Quit => self.game_over = true,
            Pick => {
                let (msg,over) = self.game_board.pick(action.get_args());
                self.game_over = over;
                return Ok(msg);
            }
        };
        Ok(format!("Debug<{}>", action).to_string())
    }
}

impl ActionParser for Minesweeper {
    type ActionThing = UserAction;
    type UserInputThing = UserInput;

    fn parse(input: Self::UserInputThing) -> Self::ActionThing {
        UserAction::from(&input[..])
    }

    fn try_parse(input: Self::UserInputThing) -> Option<Self::ActionThing> {
        Some(UserAction::from(&input[..]))
    }
}

impl ConfigHandler<Number,Number> for Minesweeper {
    type Key = Number;
    type Value = Number;
    type ConfigThing = Settings;
    type UserInputThing = UserInput;
    type MessageThing = Message;

    fn try_get(_config: &Self::ConfigThing, _ukey: Self::UserInputThing) -> Result<&Number, ConfigHandlerError> {
        Err(ConfigHandlerError::IllegalKeyError)
    }

    fn try_set(_config: &mut Self::ConfigThing, _key: Self::UserInputThing, _val: Self::UserInputThing) -> Result<&Number,ConfigHandlerError> {
        Err(ConfigHandlerError::IllegalValueError)
    }
}

impl ConfigParser<Number> for UserInput {
    fn parse(_s: Self) -> Number {
        0
    }

    fn try_parse(_s: Self) -> Option<Number> {
        Some(0)
    }
}