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
    object.kitty_image_protocol = false

    return object
end

function twk.config:with_format(format_builder)
    self.format_config = format_builder
    return self
end

twk.template = {}

function twk.template:new()
    local object = {}
    setmetatable(object, self)
    self.__index = self

    object.stack = {}

    return object
end

function twk.template:text(string)
    table.insert(self.stack, { type = "text", string = string })
    return self
end

function twk.template:evaluated_text(func)
    table.insert(self.stack, { type = "eval_text", func = func })
    return self
end

function twk.template:placeholder(reference)
    table.insert(self.stack, { type = "placeholder", reference = reference })
    return self
end

function twk.template:color(color)
    table.insert(self.stack, { type = "color", color = color })
    return self
end

twk.border = {}

function twk.border:new()
    local object = {}
    setmetatable(object, self)
    self.__index = self
    return object
end

function twk.border:with_top_left_corner(symbol)
    self.top_left_corner = symbol
    return self
end

function twk.border:with_top_right_corner(symbol)
    self.top_right_corner = symbol
    return self
end

function twk.border:with_bottom_left_corner(symbol)
    self.bottom_left_corner = symbol
    return self
end

function twk.border:with_bottom_right_corner(symbol)
    self.bottom_right_corner = symbol
    return self
end

function twk.border:with_top_edge(symbol)
    self.top_edge = symbol
    return self
end

function twk.border:with_right_edge(symbol)
    self.right_edge = symbol
    return self
end

function twk.border:with_bottom_edge(symbol)
    self.bottom_edge = symbol
    return self
end

function twk.border:with_left_edge(symbol)
    self.left_edge = symbol
    return self
end

twk.format = {}

function twk.format:new()
    local object = {}
    setmetatable(object, self)
    self.__index = self
    return self
end

function twk.format:key(template)
    self.key_format = template
    return self
end

function twk.format:arrow(template)
    self.arrow_format = template
    return self
end

function twk.format:name(template)
    self.name_format = template
    return self
end

function twk.format:menu(template)
    self.arrow_format = template
    return self
end

function twk.format:border(border_builder)
    self.border_format = border_builder
    return self
end

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
