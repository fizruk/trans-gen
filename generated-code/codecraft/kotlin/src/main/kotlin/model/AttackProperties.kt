package model

import util.StreamUtil

class AttackProperties {
    var attackRange: Int = 0
    var damage: Int = 0
    var collectResource: Boolean = false

    constructor(attackRange: Int, damage: Int, collectResource: Boolean) {
        this.attackRange = attackRange
        this.damage = damage
        this.collectResource = collectResource
    }

    @Throws(java.io.IOException::class)
    fun writeTo(stream: java.io.OutputStream) {
        StreamUtil.writeInt(stream, attackRange)
        StreamUtil.writeInt(stream, damage)
        StreamUtil.writeBoolean(stream, collectResource)
    }

    override fun toString(): String {
        var stringBuilder = StringBuilder("AttackProperties { ")
        stringBuilder.append("attackRange: ")
        stringBuilder.append(attackRange)
        stringBuilder.append(", ")
        stringBuilder.append("damage: ")
        stringBuilder.append(damage)
        stringBuilder.append(", ")
        stringBuilder.append("collectResource: ")
        stringBuilder.append(collectResource)
        stringBuilder.append(" }")
        return stringBuilder.toString()
    }

    companion object {
        @Throws(java.io.IOException::class)
        fun readFrom(stream: java.io.InputStream): AttackProperties {
            var attackRange: Int
            attackRange = StreamUtil.readInt(stream)
            var damage: Int
            damage = StreamUtil.readInt(stream)
            var collectResource: Boolean
            collectResource = StreamUtil.readBoolean(stream)
            return AttackProperties(attackRange, damage, collectResource)
        }
    }
}