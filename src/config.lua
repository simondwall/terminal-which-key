local twk = {}

twk.set_config = function(config_builder)
    twk.configuration = config_builder
end

twk.config = {}

function twk.config:new(root_menu, shell_path)
    local object = {}
    setmetatable(object, self)
    self.__index = self

    object.menu_config = root_menu
    object.shell_path = shell_path

    return object
end

twk.border = {}

twk.menu = {}

function twk.menu:new(key, name)
    local object = {}
    setmetatable(object, self)
    self.__index = self

    object.key = key
    object.name = name

    object.children = {}

    return object
end

function twk.menu:with_condition(condition)
    self.condition = condition
    return self
end

function twk.menu:with_description(description)
    self.description = description
    return self
end

function twk.menu:add_action(action_builder)
    table.insert(self.children, { type = "action", action = action_builder })
    return self
end

function twk.menu:add_menu(menu_builder)
    table.insert(self.children, { type = "menu", menu = menu_builder })
    return self
end

twk.action = {}

function twk.action:new(key, name, action)
    local object = {}
    setmetatable(object, self)
    self.__index = self

    object.key = key
    object.name = name
    object.action = action

    return object
end

function twk.action:with_condition(condition)
    self.condition = condition
    return self
end

function twk.action:with_description(description)
    self.description = description
    return self
end

return twk
