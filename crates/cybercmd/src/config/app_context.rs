use common::{
    extensions::{Extensions, PathExt},
    make_path,
    path::{Error as PathError, PathBuf},
    setup,
};
use log::info;

use super::{ArgumentContext, GameConfigList};

pub struct Paths {
    pub game: PathBuf,
    pub logs: PathBuf,
    pub configs: PathBuf,
    pub tools: PathBuf,
    pub scc: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppContext {
    pub paths: Paths,
    pub game_configs: GameConfigList,
    pub argument_context: ArgumentContext,
}

impl Paths {
    fn new() -> Paths {
        let game = Paths::get_game_path().unwrap();

        Paths {
            logs: make_path!(&game, "r6", "logs"),
            configs: make_path!(&game, "r6", "config", "cybercmd"),
            tools: make_path!(&game, "tools", "cybercmd"),
            scc: make_path!(&game, "engine", "tools", "scc.exe"),
            game,
        }
    }

    fn get_game_path() -> Result<PathBuf, PathError> {
        let game_path = std::env::current_exe()?
            .normalize()?
            .ancestors()
            .nth(3)
            .ok_or(PathError::NoParent)?
            .normalize_virtually()?;

        Ok(game_path)
    }
}

impl AppContext {
    /// # Errors
    /// Returns `anyhow::Error` aggregating many error types.
    pub fn new() -> anyhow::Result<AppContext> {
        let paths = Paths::new();
        setup(&paths.logs)?;
        info!("Loading Cybercmd");

        let app_context = AppContext {
            game_configs: GameConfigList::new(&paths)?,
            argument_context: ArgumentContext::new(&paths),
            paths,
        };
        Ok(app_context)
    }
}
