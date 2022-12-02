use mlua::{FromLua, Function, Lua, Result, Table, Value};

use crate::keys::Key;

#[derive(Debug)]
pub struct Config<'lua> {
    pub menu_config: MenuConfig<'lua>,
    pub shell_path: String,
}

impl<'lua> FromLua<'lua> for Config<'lua> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let table: Table = lua.unpack(lua_value)?;
        let menu_config = table.get("menu_config")?;
        let shell_path = table.get("shell_path")?;
        Ok(Self {
            menu_config,
            shell_path,
        })
    }
}

#[derive(Debug)]
pub enum Child<'lua> {
    Action(KeyConfig<'lua>),
    Menu(MenuConfig<'lua>),
}

impl<'lua> FromLua<'lua> for Child<'lua> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let table: Table = lua.unpack(lua_value)?;
        let content_type: String = table.get("type")?;
        match content_type.as_str() {
            "action" => Ok(Child::Action(table.get("action")?)),
            "menu" => Ok(Child::Menu(table.get("menu")?)),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "Table",
                to: "Child",
                message: None,
            }),
        }
    }
}

#[derive(Debug)]
pub struct MenuConfig<'lua> {
    pub key: Key,
    pub name: String,
    pub children: Vec<Child<'lua>>,
    pub condition: Option<Function<'lua>>,
    pub description: Option<String>,
}

impl<'lua> FromLua<'lua> for MenuConfig<'lua> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let table: Table = lua.unpack(lua_value)?;
        let key_string: String = table.get("key")?;
        let key = Key::from_str(&key_string).unwrap();
        let name = table.get("name")?;
        let children = table.get("children")?;
        let condition = table.get("condition").unwrap_or(None);
        let description = table.get("description").unwrap_or(None);
        return Ok(Self {
            key,
            name,
            children,
            condition,
            description,
        });
    }
}

#[derive(Debug)]
pub struct KeyConfig<'lua> {
    pub key: Key,
    pub name: String,
    pub action: Function<'lua>,
    pub condition: Option<Function<'lua>>,
    pub description: Option<String>,
}

impl<'lua> FromLua<'lua> for KeyConfig<'lua> {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
        let table: Table = lua.unpack(lua_value)?;
        let key_string: String = table.get("key")?;
        let key = Key::from_str(&key_string).unwrap();
        let name = table.get("name")?;
        let action = table.get("action")?;
        let condition = table.get("condition").unwrap_or(None);
        let description = table.get("description").unwrap_or(None);
        return Ok(Self {
            key,
            name,
            action,
            condition,
            description,
        });
    }
}

pub fn load_config_from_file(path: &str) -> Result<Config<'static>> {
    let lua_twk = include_str!("config.lua");
    let lua_config = std::fs::read(path).expect(&format!("Could not read {path}!"));

    let lua = Lua::new().into_static();

    let twk: Table = lua.load(lua_twk).call(()).unwrap();
    lua.globals().set("twk", twk).unwrap();

    lua.load(&lua_config).call::<_, ()>(()).unwrap();
    let twk: Table = lua.globals().get("twk").unwrap();

    let config: Config = twk.get("configuration").unwrap();

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        keys::{Key, Symbol},
    };
    use mlua::{Lua, Table};

    #[test]
    fn simple_config() {
        // +-------+
        // | Setup |
        // +-------+

        // load lua files
        let lua_twk = include_str!("config.lua");
        let lua_config = include_str!("../test_files/simple_config.lua");

        // create lua environment
        let lua = Lua::new();

        // load twk into environment
        let twk: Table = lua.load(lua_twk).call(()).unwrap();
        lua.globals().set("twk", twk).unwrap();

        // get config table
        lua.load(lua_config).call::<_, ()>(()).unwrap();
        let twk: Table = lua.globals().get("twk").unwrap();

        // +-----+
        // | Run |
        // +-----+

        // deserialize config table
        let config: Config = twk.get("configuration").unwrap();

        // +--------+
        // | Assert |
        // +--------+

        // check some values from config
        assert_eq!(config.shell_path, "/opt/homebrew/bin/fish");
        assert_eq!(config.menu_config.name, "menu");
        assert_eq!(
            config.menu_config.key,
            Key {
                ctrl: true,
                opt: false,
                shift: true,
                symbol: Symbol::A
            }
        );
        assert_eq!(
            config.menu_config.description.unwrap(),
            "This is just for testing"
        );
    }

    #[test]
    fn complex_config() {
        // Setup
        let lua_twk = include_str!("config.lua");
        let lua_config = include_str!("../test_files/complex_config.lua");

        let lua = Lua::new();

        let twk: Table = lua.load(lua_twk).call(()).unwrap();
        lua.globals().set("twk", twk).unwrap();

        lua.load(lua_config).call::<_, ()>(()).unwrap();
        let twk: Table = lua.globals().get("twk").unwrap();

        // Run
        let config: Config = twk.get("configuration").unwrap();

        // Assert
        assert_eq!(config.shell_path, "/opt/homebrew/bin/fish")
    }

    #[test]
    fn minimal_two_menus_config() {
        // Setup
        let lua_twk = include_str!("config.lua");
        let lua_config = include_str!("../test_files/minimal_two_menus.lua");

        let lua = Lua::new();

        let twk: Table = lua.load(lua_twk).call(()).unwrap();
        lua.globals().set("twk", twk).unwrap();

        lua.load(lua_config).call::<_, ()>(()).unwrap();
        let twk: Table = lua.globals().get("twk").unwrap();

        // Run
        let config: Config = twk.get("configuration").unwrap();

        // Assert
        assert_eq!(config.shell_path, "/opt/homebrew/bin/fish")
    }
}