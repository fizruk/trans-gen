from stream_wrapper import StreamWrapper
from typing import List

class OneOf:
    """Oneof example"""

    @staticmethod
    def read_from(stream: StreamWrapper) -> "OneOf":
        """Read OneOf from input stream
        """
        tag = stream.read_int()
        if tag == OptionOne.TAG:
            return OneOf.OptionOne.read_from(stream)
        if tag == OptionTwo.TAG:
            return OneOf.OptionTwo.read_from(stream)
        raise Exception("Unexpected tag value")

class OptionOne(OneOf):
    """First option"""

    TAG = 0

    __slots__ = ("vec_int","long_int",)

    vec_int: List[int]
    long_int: int

    def __init__(self, vec_int: List[int], long_int: int):
        self.vec_int = vec_int
        """List of integers"""
        self.long_int = long_int
        """Long integer"""

    @staticmethod
    def read_from(stream: StreamWrapper) -> "OptionOne":
        """Read OptionOne from input stream
        """
        vec_int = []
        for _ in range(stream.read_int()):
            vec_int_element = stream.read_int()
            vec_int.append(vec_int_element)
        long_int = stream.read_long()
        return OptionOne(vec_int, long_int)
    
    def write_to(self, stream: StreamWrapper):
        """Write OptionOne to output stream
        """
        stream.write_int(self.TAG)
        stream.write_int(len(self.vec_int))
        for element in self.vec_int:
            stream.write_int(element)
        stream.write_long(self.long_int)
    
    def __repr__(self):
        return "OptionOne(" + \
            repr(self.vec_int) + \
            ", " + \
            repr(self.long_int) + \
            ")"

OneOf.OptionOne = OptionOne

class OptionTwo(OneOf):
    """Second option"""

    TAG = 1

    __slots__ = ("value",)

    value: int

    def __init__(self, value: int):
        self.value = value
        """usize"""

    @staticmethod
    def read_from(stream: StreamWrapper) -> "OptionTwo":
        """Read OptionTwo from input stream
        """
        value = stream.read_int()
        return OptionTwo(value)
    
    def write_to(self, stream: StreamWrapper):
        """Write OptionTwo to output stream
        """
        stream.write_int(self.TAG)
        stream.write_int(self.value)
    
    def __repr__(self):
        return "OptionTwo(" + \
            repr(self.value) + \
            ")"

OneOf.OptionTwo = OptionTwo