use config::lua::get_or_create_module;
use config::lua::mlua::{self, Lua};

pub fn register(lua: &Lua) -> anyhow::Result<()> {
    let wezterm_mod = get_or_create_module(lua, "wezterm")?;
    // TODO: multithreading safety? https://news.ycombinator.com/item?id=13528407
    wezterm_mod.set(
        "setenv",
        lua.create_function(|_: &Lua, (key, value): (String, String)| {
            std::panic::catch_unwind(|| std::env::set_var(key, value))
                .map_err(|e| mlua::Error::external(e.downcast_ref::<&str>().unwrap().to_owned()))
        })?,
    )?;
    Ok(())
}
