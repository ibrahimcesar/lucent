-- Lua example file

--[[
  Multi-line comment block
  This is a simple calculator
]]

-- Global function
function add(a, b)
    return a + b
end

-- Local function
local function multiply(a, b)
    return a * b
end

-- Table with methods
local Calculator = {}

function Calculator.divide(a, b)
    if b == 0 then
        return nil
    end
    return a / b
end

return Calculator
