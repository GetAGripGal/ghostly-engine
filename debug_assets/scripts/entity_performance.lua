---
--- A performance test for the entity system of the engine.
---

SPAWN_ITER = 0.05
local iter = 0

local spawn_count = 0
local velocity = {
	x = 600,
	y = 400
}
local running = true

-- Spawn a new entity
local function spawn_entity(world)
	local id = world:spawn()
	local entity = world:get_entity(id)

	entity.role = "performance"
	entity.position = {
		x = (spawn_count * 64),
		y = -64,
		z = 0
	}
end

-- Move an entity
local function move_entity(entity, bound_x, bound_y, delta)
	if not entity.role == "performance" then
		return
	end
	local position = entity.position

	if position.x > bound_x + 64 then
		position.x = -64
	end
	if position.y > bound_y then
		position.y = -64
	end

	position.x = position.x + velocity.x * delta
	position.y = position.y + velocity.y * delta
	entity.position = position
end

-- Move each entity in the world
local function move_entities(api)
	local input = api:input()

	if input:key_pressed("space") then
		running = not running
	end

	if not running then
		return
	end

	local performance = api:performance()
	local delta = performance:delta_secs()

	local window = api:window()
	local world = api:world()

	iter = iter + delta

	if iter >= SPAWN_ITER then
		iter = 0
		spawn_count = spawn_count + 1
		spawn_entity(world)

		-- If the fps drops below 30 the test is done
		if performance:fps() <= 30 then
			running = false
			return
		end
	end

	if world:entity_count() <= 0 then
		return
	end

	-- Move each entity
	for i = 1, world:entity_count(), 1 do
		local entity = world:get_entity(i - 1)
		move_entity(entity, window:width(), window:height(), delta)
	end
end

return function(api)
	local window = api:window()
	window:set_title(string.format("Entity Performance"))

	local systems = api:systems()
	systems:register_update(move_entities)
end
