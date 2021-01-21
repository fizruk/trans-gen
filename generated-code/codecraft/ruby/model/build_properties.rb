require_relative 'entity_type'

class BuildProperties
    attr_accessor :options
    attr_accessor :init_health

    def initialize(options, init_health)
        @options = options
        @init_health = init_health
    end

    def self.read_from(stream)
        options = []
        stream.read_int().times do |_|
            options_element = EntityType.read_from(stream)
            options.push(options_element)
        end
        if stream.read_bool()
            init_health = stream.read_int()
        else
            init_health = nil
        end
        BuildProperties.new(options, init_health)
    end

    def write_to(stream)
        stream.write_int(@options.length())
        @options.each do |options_element|
            stream.write_int(options_element)
        end
        if @init_health.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            stream.write_int(@init_health)
        end
    end
end