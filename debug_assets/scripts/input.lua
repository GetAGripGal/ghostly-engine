---
--- A simple test script to test the input api.
---
PLAYER_SIZE = 64

local player_id = -1
local player_speed = 200
local player_run_boost = 1.50 -- 50% speed increase
local player_jump = 2400
local player_friction = 8
local gravity = 80
local on_floor = false

local can_jump = true

local velocity = {
	x = 0,
	y = 0,
}

--- Linear interpolation function
local function lerp(a, b, t) return a * (1 - t) + b * t end

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

--- Apply collision to the player
local function apply_collision(position, x_bounds, y_bounds)
	on_floor = false
	if position.y + PLAYER_SIZE > y_bounds then
		position.y = y_bounds - PLAYER_SIZE
		velocity.y = 0
		on_floor = true
	end
	if position.x < 0 then
		position.x = 0
		velocity.x = 0
	end
	if position.x + PLAYER_SIZE > x_bounds then
		position.x = x_bounds - PLAYER_SIZE
		velocity.x = 0
	end
end

--- Handle the walking action
local function handle_walking(input)
	local x_axis = input:get_axis("a", "d")
	local speed = player_speed * (input:key_down("left_shift") and player_run_boost or 1)

	velocity.x = velocity.x + x_axis * speed
end

--- Handle the jumping action
local function handle_jump(input)
	if input:key_down("space") then
		if on_floor and can_jump then
			velocity.y = velocity.y - player_jump
			can_jump = false
		end
	else
		can_jump = true
	end
end

--- Apply the forces to the player
local function apply_forces(delta)
	velocity.y = velocity.y + gravity
	velocity.x = lerp(velocity.x, 0, player_friction * delta)
end

--- Apply the velocity
local function apply_velocity(position, delta)
	position.x = position.x + velocity.x * delta
	position.y = position.y + velocity.y * delta
end

--- Move the player entity based on user input.
local function system_move_player(api)
	local world = api:world()
	local input = api:input()
	local delta = api:performance():delta_secs()
	local window = api:window()

	-- Skip if the player isnt spawned yet.
	if player_id < 0 then
		return
	end

	local player = world:get_entity(player_id)
	local position = player.position

	handle_walking(input)
	apply_forces(delta)
	handle_jump(input)
	apply_velocity(position, delta)
	apply_collision(position, window:width(), window:height())

	player.position = position
end

--- Initialize the script.
local function init(api)
	local window = api:window()
	window:set_title("Input Test")

	spawn_player(api:world(), window)

	local systems = api:systems()
	systems:register_update(system_move_player)
end

return init
