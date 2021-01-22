package model

import util.StreamUtil

class Example {
    lateinit var oneOf: model.OneOf
    lateinit var hashMap: MutableMap<model.Enumeration, Int>
    var optionalInt: Int? = null
    var optionalBoolean: Boolean? = null
    var optionalOneOf: model.OneOf? = null
    var optionalStruct: model.Structure? = null
    var optionalEnum: model.Enumeration? = null

    constructor(oneOf: model.OneOf, hashMap: MutableMap<model.Enumeration, Int>, optionalInt: Int?, optionalBoolean: Boolean?, optionalOneOf: model.OneOf?, optionalStruct: model.Structure?, optionalEnum: model.Enumeration?) {
        this.oneOf = oneOf
        this.hashMap = hashMap
        this.optionalInt = optionalInt
        this.optionalBoolean = optionalBoolean
        this.optionalOneOf = optionalOneOf
        this.optionalStruct = optionalStruct
        this.optionalEnum = optionalEnum
    }

    @Throws(java.io.IOException::class)
    fun writeTo(stream: java.io.OutputStream) {
        oneOf.writeTo(stream)
        StreamUtil.writeInt(stream, hashMap.size)
        for (hashMapEntry in hashMap) {
            val hashMapKey = hashMapEntry.key
            StreamUtil.writeInt(stream, hashMapKey.tag)
            val hashMapValue = hashMapEntry.value
            StreamUtil.writeInt(stream, hashMapValue)
        }
        val optionalIntValue = optionalInt
        if (optionalIntValue == null) {
            StreamUtil.writeBoolean(stream, false)
        } else {
            StreamUtil.writeBoolean(stream, true)
            StreamUtil.writeInt(stream, optionalIntValue)
        }
        val optionalBooleanValue = optionalBoolean
        if (optionalBooleanValue == null) {
            StreamUtil.writeBoolean(stream, false)
        } else {
            StreamUtil.writeBoolean(stream, true)
            StreamUtil.writeBoolean(stream, optionalBooleanValue)
        }
        val optionalOneOfValue = optionalOneOf
        if (optionalOneOfValue == null) {
            StreamUtil.writeBoolean(stream, false)
        } else {
            StreamUtil.writeBoolean(stream, true)
            optionalOneOfValue.writeTo(stream)
        }
        val optionalStructValue = optionalStruct
        if (optionalStructValue == null) {
            StreamUtil.writeBoolean(stream, false)
        } else {
            StreamUtil.writeBoolean(stream, true)
            optionalStructValue.writeTo(stream)
        }
        val optionalEnumValue = optionalEnum
        if (optionalEnumValue == null) {
            StreamUtil.writeBoolean(stream, false)
        } else {
            StreamUtil.writeBoolean(stream, true)
            StreamUtil.writeInt(stream, optionalEnumValue.tag)
        }
    }

    companion object {
        @Throws(java.io.IOException::class)
        fun readFrom(stream: java.io.InputStream): Example {
            var oneOf: model.OneOf
            oneOf = model.OneOf.readFrom(stream)
            var hashMap: MutableMap<model.Enumeration, Int>
            val hashMapSize = StreamUtil.readInt(stream)
            hashMap = mutableMapOf();
            for (hashMapIndex in 0 until hashMapSize) {
                var hashMapKey: model.Enumeration
                hashMapKey = model.Enumeration.readFrom(stream)
                var hashMapValue: Int
                hashMapValue = StreamUtil.readInt(stream)
                hashMap.put(hashMapKey, hashMapValue)
            }
            var optionalInt: Int?
            if (StreamUtil.readBoolean(stream)) {
                optionalInt = StreamUtil.readInt(stream)
            } else {
                optionalInt = null
            }
            var optionalBoolean: Boolean?
            if (StreamUtil.readBoolean(stream)) {
                optionalBoolean = StreamUtil.readBoolean(stream)
            } else {
                optionalBoolean = null
            }
            var optionalOneOf: model.OneOf?
            if (StreamUtil.readBoolean(stream)) {
                optionalOneOf = model.OneOf.readFrom(stream)
            } else {
                optionalOneOf = null
            }
            var optionalStruct: model.Structure?
            if (StreamUtil.readBoolean(stream)) {
                optionalStruct = model.Structure.readFrom(stream)
            } else {
                optionalStruct = null
            }
            var optionalEnum: model.Enumeration?
            if (StreamUtil.readBoolean(stream)) {
                optionalEnum = model.Enumeration.readFrom(stream)
            } else {
                optionalEnum = null
            }
            return Example(oneOf, hashMap, optionalInt, optionalBoolean, optionalOneOf, optionalStruct, optionalEnum)
        }
    }
}