require_relative 'entity_type'
require_relative 'vec2_int'

class Entity
    attr_accessor :id
    attr_accessor :player_id
    attr_accessor :entity_type
    attr_accessor :position
    attr_accessor :health
    attr_accessor :active

    def initialize(id, player_id, entity_type, position, health, active)
        @id = id
        @player_id = player_id
        @entity_type = entity_type
        @position = position
        @health = health
        @active = active
    end

    def self.read_from(stream)
        id = stream.read_int()
        if stream.read_bool()
            player_id = stream.read_int()
        else
            player_id = nil
        end
        entity_type = EntityType.read_from(stream)
        position = Vec2Int.read_from(stream)
        health = stream.read_int()
        active = stream.read_bool()
        Entity.new(id, player_id, entity_type, position, health, active)
    end

    def write_to(stream)
        stream.write_int(@id)
        if @player_id.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            stream.write_int(@player_id)
        end
        stream.write_int(@entity_type)
        @position.write_to(stream)
        stream.write_int(@health)
        stream.write_bool(@active)
    end

    def to_s
        string_result = "Entity { "
        string_result += "id: "
        string_result += @id.to_s
        string_result += ", "
        string_result += "player_id: "
        if @player_id.nil?
            string_result += "nil"
        else
            string_result += @player_id.to_s
        end
        string_result += ", "
        string_result += "entity_type: "
        string_result += EntityType.to_s(@entity_type)
        string_result += ", "
        string_result += "position: "
        string_result += @position.to_s
        string_result += ", "
        string_result += "health: "
        string_result += @health.to_s
        string_result += ", "
        string_result += "active: "
        string_result += @active.to_s
        string_result += " }"
        string_result
    end

    def to_str
        to_s
    end
end