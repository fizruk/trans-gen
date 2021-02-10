from model.entity_action import EntityAction

class Action:
    """Player's action"""

    def __init__(self, entity_actions):
        self.entity_actions = entity_actions
        """New actions for entities. If entity does not get new action, if will continue to perform previously set one"""

    @staticmethod
    def read_from(stream):
        """Read Action from input stream
        """
        entity_actions = {}
        for _ in range(stream.read_int()):
            entity_actions_key = stream.read_int()
            entity_actions_value = EntityAction.read_from(stream)
            entity_actions[entity_actions_key] = entity_actions_value
        return Action(entity_actions)

    def write_to(self, stream):
        """Write Action to output stream
        """
        stream.write_int(len(self.entity_actions))
        for key, value in self.entity_actions.items():
            stream.write_int(key)
            value.write_to(stream)

    def __repr__(self):
        return "Action(" + \
            repr(self.entity_actions) + \
            ")"