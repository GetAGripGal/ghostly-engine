Player_id = -1

function Init(world)
	Player_Id = world:spawn()
	local entity = world:get_entity(Player_Id)

	entity.role = "player"
	entity.position = {
		x = 320,
		y = 240,
		z = 0
	}
end

function Update(world)
	local entity = world:get_entity(Player_Id)
	local position = entity.position

	if position.x > 1280 + 64 then
		position.x = 0
	end
	if position.y > 720 then
		position.y = 0
	end

	position.x = position.x + 3
	position.y = position.y + 2
	entity.position = position
end
