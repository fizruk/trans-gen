require_relative 'enumeration'
require_relative 'one_of'
require_relative 'structure'

class Example
    attr_accessor :one_of
    attr_accessor :hash_map
    attr_accessor :optional_int
    attr_accessor :optional_bool
    attr_accessor :optional_one_of
    attr_accessor :optional_struct
    attr_accessor :optional_enum

    def initialize(one_of, hash_map, optional_int, optional_bool, optional_one_of, optional_struct, optional_enum)
        @one_of = one_of
        @hash_map = hash_map
        @optional_int = optional_int
        @optional_bool = optional_bool
        @optional_one_of = optional_one_of
        @optional_struct = optional_struct
        @optional_enum = optional_enum
    end

    def self.read_from(stream)
        one_of = OneOf.read_from(stream)
        hash_map = Hash.new
        stream.read_int().times do |_|
            hash_map_key = Enumeration.read_from(stream)
            hash_map_value = stream.read_int()
            hash_map[hash_map_key] = hash_map_value
        end
        if stream.read_bool()
            optional_int = stream.read_int()
        else
            optional_int = nil
        end
        if stream.read_bool()
            optional_bool = stream.read_bool()
        else
            optional_bool = nil
        end
        if stream.read_bool()
            optional_one_of = OneOf.read_from(stream)
        else
            optional_one_of = nil
        end
        if stream.read_bool()
            optional_struct = Structure.read_from(stream)
        else
            optional_struct = nil
        end
        if stream.read_bool()
            optional_enum = Enumeration.read_from(stream)
        else
            optional_enum = nil
        end
        Example.new(one_of, hash_map, optional_int, optional_bool, optional_one_of, optional_struct, optional_enum)
    end

    def write_to(stream)
        @one_of.write_to(stream)
        stream.write_int(@hash_map.length())
        @hash_map.each do |hash_map_key, hash_map_value|
            stream.write_int(hash_map_key)
            stream.write_int(hash_map_value)
        end
        if @optional_int.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            stream.write_int(@optional_int)
        end
        if @optional_bool.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            stream.write_bool(@optional_bool)
        end
        if @optional_one_of.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            @optional_one_of.write_to(stream)
        end
        if @optional_struct.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            @optional_struct.write_to(stream)
        end
        if @optional_enum.nil?
            stream.write_bool(false)
        else
            stream.write_bool(true)
            stream.write_int(@optional_enum)
        end
    end
end