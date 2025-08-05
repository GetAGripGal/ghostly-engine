---
--- A simple test script to test the input api.
---

local player_id = -1
local player_speed = 800

--- Spawn the player entity.
local function spawn_player(world, window)
	player_id = world:spawn()
	local entity = world:get_entity(player_id)

	entity.role = "player"
	entity.position = {
		x = window:width() / 2,
		y = window:height() / 2,
		z = 0
	}
end

--- Move the player entity based on user input.
local function move_player(api)
	local world = api:world()
	local input = api:input()
	local delta = api:performance():delta_secs()

	-- Skip if the player isnt spawned yet.
	if player_id < 0 then
		return
	end

	local player = world:get_entity(player_id)
	local position = player.position

	local x_axis = input:get_axis("a", "d")
	local y_axis = input:get_axis("w", "s")

	position.x = position.x + x_axis * player_speed * delta
	position.y = position.y + y_axis * player_speed * delta

	player.position = position
end

--- Initialize the script.
local function init(api)
	local window = api:window()
	window:set_title("Input Test")

	spawn_player(api:world(), window)

	local systems = api:systems()
	systems:register_update(move_player)
end

return init
