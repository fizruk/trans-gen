from model.player_view import PlayerView

class ServerMessage:
    """Message sent from server"""

    @staticmethod
    def read_from(stream):
        """Read ServerMessage from input stream
        """
        tag = stream.read_int()
        if tag == GetAction.TAG:
            return ServerMessage.GetAction.read_from(stream)
        if tag == Finish.TAG:
            return ServerMessage.Finish.read_from(stream)
        if tag == DebugUpdate.TAG:
            return ServerMessage.DebugUpdate.read_from(stream)
        raise Exception("Unexpected tag value")

class GetAction(ServerMessage):
    """Get action for next tick"""

    TAG = 0

    def __init__(self, player_view, debug_available):
        self.player_view = player_view
        """Player's view"""
        self.debug_available = debug_available
        """Whether app is running with debug interface available"""

    @staticmethod
    def read_from(stream):
        """Read GetAction from input stream
        """
        player_view = PlayerView.read_from(stream)
        debug_available = stream.read_bool()
        return GetAction(player_view, debug_available)

    def write_to(self, stream):
        """Write GetAction to output stream
        """
        stream.write_int(self.TAG)
        self.player_view.write_to(stream)
        stream.write_bool(self.debug_available)

    def __repr__(self):
        return "GetAction(" + \
            repr(self.player_view) + \
            ", " + \
            repr(self.debug_available) + \
            ")"

ServerMessage.GetAction = GetAction

class Finish(ServerMessage):
    """Signifies end of the game"""

    TAG = 1

    def __init__(self):
        pass

    @staticmethod
    def read_from(stream):
        """Read Finish from input stream
        """
        return Finish()

    def write_to(self, stream):
        """Write Finish to output stream
        """
        stream.write_int(self.TAG)

    def __repr__(self):
        return "Finish(" + \
            ")"

ServerMessage.Finish = Finish

class DebugUpdate(ServerMessage):
    """Debug update"""

    TAG = 2

    def __init__(self, player_view):
        self.player_view = player_view
        """Player's view"""

    @staticmethod
    def read_from(stream):
        """Read DebugUpdate from input stream
        """
        player_view = PlayerView.read_from(stream)
        return DebugUpdate(player_view)

    def write_to(self, stream):
        """Write DebugUpdate to output stream
        """
        stream.write_int(self.TAG)
        self.player_view.write_to(stream)

    def __repr__(self):
        return "DebugUpdate(" + \
            repr(self.player_view) + \
            ")"

ServerMessage.DebugUpdate = DebugUpdate