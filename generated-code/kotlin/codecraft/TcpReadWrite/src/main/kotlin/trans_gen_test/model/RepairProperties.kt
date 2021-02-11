package trans_gen_test.model

import trans_gen_test.util.StreamUtil

/**
 * Entity's repair properties
 */
class RepairProperties {
    /**
     * Valid target entity types
     */
    var validTargets: Array<trans_gen_test.model.EntityType>
    /**
     * Health restored in one tick
     */
    var power: Int

    constructor(validTargets: Array<trans_gen_test.model.EntityType>, power: Int) {
        this.validTargets = validTargets
        this.power = power
    }

    /**
     * Write RepairProperties to output stream
     */
    @Throws(java.io.IOException::class)
    fun writeTo(stream: java.io.OutputStream) {
        StreamUtil.writeInt(stream, validTargets.size)
        for (validTargetsElement in validTargets) {
            StreamUtil.writeInt(stream, validTargetsElement.tag)
        }
        StreamUtil.writeInt(stream, power)
    }

    /**
     * Get string representation of RepairProperties
     */
    override fun toString(): String {
        var stringBuilder = StringBuilder("RepairProperties { ")
        stringBuilder.append("validTargets: ")
        stringBuilder.append("[ ")
        var validTargetsIndex = 0
        for (validTargetsElement in validTargets) {
            if (validTargetsIndex != 0) {
                stringBuilder.append(", ")
            }
            stringBuilder.append(validTargetsElement)
            validTargetsIndex++
        }
        stringBuilder.append(" ]")
        stringBuilder.append(", ")
        stringBuilder.append("power: ")
        stringBuilder.append(power)
        stringBuilder.append(" }")
        return stringBuilder.toString()
    }

    companion object {
        /**
         * Read RepairProperties from input stream
         */
        @Throws(java.io.IOException::class)
        fun readFrom(stream: java.io.InputStream): RepairProperties {
            var validTargets: Array<trans_gen_test.model.EntityType>
            validTargets = Array(StreamUtil.readInt(stream), {
                var validTargetsElement: trans_gen_test.model.EntityType
                validTargetsElement = trans_gen_test.model.EntityType.readFrom(stream)
                validTargetsElement
            })
            var power: Int
            power = StreamUtil.readInt(stream)
            return RepairProperties(validTargets, power)
        }
    }
}