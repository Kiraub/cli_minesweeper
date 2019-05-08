
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
            ActionParser,
            ActionTypeT
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

    pub fn start(&mut self) -> Message {
        let width = self.game_settings.get(0);
        let height = self.game_settings.get(1);
        let bombs = self.game_settings.get(3);
        match self.game_board.init(width, height, bombs) {
            Ok(_) => format!("Started new game with {}x{} field and {} bombs.", width, height, bombs),
            Err(e) => e
        }
    }

    pub fn show_board(&self) {
        print!("{}", self.game_board);
    }

    pub fn handle_input(&mut self, input: UserInput) -> Result<Message,&'static str> {
        self.do_action(Minesweeper::parse(input))
    }

    pub fn config(&mut self, args: &Vec<String>) -> Message {
        let length = args.len();
        if length == 0 {
            "Not enough arguments given to action.".to_string()
        } else if length == 1 {
            match Minesweeper::try_get(&self.game_settings, &args[0]) {
                Ok(val) => format!("Value of {} is {}.", args[0], val),
                Err(_) => "Unknown config key.".to_string()
            }
        } else if length == 2  {
            if let Err(kind) = Minesweeper::try_set(&mut self.game_settings, &args[0], &args[1]) {
                match kind {
                    ConfigHandlerError::IllegalKeyError => "Unknown config key.".to_string(),
                    ConfigHandlerError::IllegalValueError => format!("Illegal value for key {}.", args[0])
                }
            } else {
                format!("Succesfully changed {} to {}.", args[0], args[1])
            }
        } else {
            "Too many arguments to config action.".to_string()
        }
    }
}

impl ActionHandler for Minesweeper {
    type ActionThing = UserAction;
    type ResultThing = Result<Message,&'static str>;

    fn do_action(&mut self, action: Self::ActionThing) -> Self::ResultThing {
        use UserActionType::*;
        match action.get_type() {
            Unknown => Err("Unknown action."),
            Reset => Ok(self.start()),
            Config => Ok(self.config(action.get_args())),
            Mark => Ok(self.game_board.mark(action.get_args())),
            Quit => {
                self.game_over = true;
                Ok("Quitting.".to_string())
            },
            Pick => {
                let (msg,over) = self.game_board.pick(action.get_args());
                self.game_over = over;
                Ok(msg)
            },
            Help => {
                let mut msg = String::from("Available actions:\n");
                for (ix, elem) in UserActionType::get_list().iter().enumerate() {
                    msg.push_str(&format!("{}:  {}\n", ix+1, elem));
                }
                Ok(msg)
            }
        }
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

    fn try_get(config: &Self::ConfigThing, ukey: &Self::UserInputThing) -> Result<Number, ConfigHandlerError> {
        if let Some(key) = ukey.try_parse() {
            if config.has(key) {
                Ok(config.get(key))
            } else {
                Err(ConfigHandlerError::IllegalKeyError)
            }
        } else {
            Err(ConfigHandlerError::IllegalKeyError)
        }
    }

    fn try_set(config: &mut Self::ConfigThing, ukey: &Self::UserInputThing, uval: &Self::UserInputThing) -> Result<(),ConfigHandlerError> {
        if let Some(key) = ukey.try_parse() {
            if config.has(key) {
                if let Ok(val) = usize::from_str_radix(&uval,10) {
                    if config.can(key,val) {
                        Ok(config.set(key,val))
                    } else {
                        Err(ConfigHandlerError::IllegalValueError)
                    }
                } else {
                    Err(ConfigHandlerError::IllegalValueError)
                }
            } else {
                Err(ConfigHandlerError::IllegalKeyError)
            }
        } else {
            Err(ConfigHandlerError::IllegalKeyError)
        }
    }
}

impl ConfigParser<Number> for UserInput {
    fn parse(&self) -> Number {
        match &self.to_lowercase()[..] {
            "w" | "width" => 0,
            "h" | "height"=> 1,
            "n" | "neighbourhood" => 2,
            "b" | "bombs" => 3,
            _ => panic!("Use parse only with valid strings. Consider try_parse for user strings.")
        }
    }

    fn try_parse(&self) -> Option<Number> {
        match &self.to_lowercase()[..] {
            "w" | "width" => Some(0),
            "h" | "height"=> Some(1),
            "n" | "neighbourhood" => Some(2),
            "b" | "bombs" => Some(3),
            _ => None
        }
    }
}