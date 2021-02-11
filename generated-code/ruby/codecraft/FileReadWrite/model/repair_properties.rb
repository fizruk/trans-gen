require './model/entity_type'

module Model

# Entity's repair properties
class RepairProperties
    # Valid target entity types
    attr_accessor :valid_targets
    # Health restored in one tick
    attr_accessor :power

    def initialize(valid_targets, power)
        @valid_targets = valid_targets
        @power = power
    end

    # Read RepairProperties from input stream
    def self.read_from(stream)
        valid_targets = []
        stream.read_int().times do |_|
            valid_targets_element = Model::EntityType.read_from(stream)
            valid_targets.push(valid_targets_element)
        end
        power = stream.read_int()
        RepairProperties.new(valid_targets, power)
    end

    # Write RepairProperties to output stream
    def write_to(stream)
        stream.write_int(@valid_targets.length())
        @valid_targets.each do |valid_targets_element|
            stream.write_int(valid_targets_element)
        end
        stream.write_int(@power)
    end

    def to_s
        string_result = "RepairProperties { "
        string_result += "valid_targets: "
        string_result += "[ "
        valid_targets_index = 0
        @valid_targets.each do |valid_targets_element|
            if valid_targets_index != 0
                string_result += ", "
            end
            string_result += EntityType.to_s(valid_targets_element)
            valid_targets_index += 1
        end
        string_result += " ]"
        string_result += ", "
        string_result += "power: "
        string_result += @power.to_s
        string_result += " }"
        string_result
    end

    def to_str
        to_s
    end
end

end