use mlua::{FromLua, Function, Lua, Result, Table, Value};

use termion::event::Key;

use crate::keys::NewFromString;

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
        let key = Key::new_from_string(&key_string);
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
        let key = Key::new_from_string(&key_string);
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

pub fn load_config_from_file(
    path: &str,
    mut master_writer: Box<dyn std::io::Write + Send>,
) -> Result<Config<'static>> {
    let lua_twk = include_str!("config.lua");
    let lua_config = std::fs::read(path).expect(&format!("Could not read {path}!"));

    let lua = Lua::new().into_static();

    let write = lua
        .create_function_mut(move |_, string: String| {
            master_writer.write_all(string.as_bytes()).unwrap();
            Ok(())
        })
        .unwrap();

    let twk: Table = lua.load(lua_twk).call(()).unwrap();
    twk.set("write", write).unwrap();
    lua.globals().set("twk", twk).unwrap();

    lua.load(&lua_config).call::<_, ()>(()).unwrap();
    let twk: Table = lua.globals().get("twk").unwrap();

    let config: Config = twk.get("configuration").unwrap();

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use mlua::{Lua, Table};

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
}
