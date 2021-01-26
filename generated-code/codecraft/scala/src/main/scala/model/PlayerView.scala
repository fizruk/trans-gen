package model

import util.StreamUtil

case class PlayerView(myId: Int, mapSize: Int, fogOfWar: Boolean, entityProperties: Map[model.EntityType, model.EntityProperties], maxTickCount: Int, maxPathfindNodes: Int, currentTick: Int, players: Seq[model.Player], entities: Seq[model.Entity]) {
    def writeTo(stream: java.io.OutputStream) {
        StreamUtil.writeInt(stream, myId)
        StreamUtil.writeInt(stream, mapSize)
        StreamUtil.writeBoolean(stream, fogOfWar)
        StreamUtil.writeInt(stream, entityProperties.size)
        entityProperties.foreach { case (key, value) =>
            key.writeTo(stream)
            value.writeTo(stream)
        }
        StreamUtil.writeInt(stream, maxTickCount)
        StreamUtil.writeInt(stream, maxPathfindNodes)
        StreamUtil.writeInt(stream, currentTick)
        StreamUtil.writeInt(stream, players.length)
        players.foreach { value =>
            value.writeTo(stream)
        }
        StreamUtil.writeInt(stream, entities.length)
        entities.foreach { value =>
            value.writeTo(stream)
        }
    }

    override def toString(): String = {
        var stringBuilder = new StringBuilder("PlayerView { ")
        stringBuilder.append("myId: ")
        stringBuilder.append(myId)
        stringBuilder.append(", ")
        stringBuilder.append("mapSize: ")
        stringBuilder.append(mapSize)
        stringBuilder.append(", ")
        stringBuilder.append("fogOfWar: ")
        stringBuilder.append(fogOfWar)
        stringBuilder.append(", ")
        stringBuilder.append("entityProperties: ")
        stringBuilder.append(entityProperties)
        stringBuilder.append(", ")
        stringBuilder.append("maxTickCount: ")
        stringBuilder.append(maxTickCount)
        stringBuilder.append(", ")
        stringBuilder.append("maxPathfindNodes: ")
        stringBuilder.append(maxPathfindNodes)
        stringBuilder.append(", ")
        stringBuilder.append("currentTick: ")
        stringBuilder.append(currentTick)
        stringBuilder.append(", ")
        stringBuilder.append("players: ")
        stringBuilder.append(players)
        stringBuilder.append(", ")
        stringBuilder.append("entities: ")
        stringBuilder.append(entities)
        stringBuilder.append(" }")
        stringBuilder.toString()
    }
}

object PlayerView {
    def readFrom(stream: java.io.InputStream): PlayerView = PlayerView(
        StreamUtil.readInt(stream),
        StreamUtil.readInt(stream),
        StreamUtil.readBoolean(stream),
        (0 until StreamUtil.readInt(stream)).map { _ => (
            model.EntityType.readFrom(stream),
            model.EntityProperties.readFrom(stream)
        )}.toMap,
        StreamUtil.readInt(stream),
        StreamUtil.readInt(stream),
        StreamUtil.readInt(stream),
        (0 until StreamUtil.readInt(stream)).map { _ =>
            model.Player.readFrom(stream)
        },
        (0 until StreamUtil.readInt(stream)).map { _ =>
            model.Entity.readFrom(stream)
        }
    )
}