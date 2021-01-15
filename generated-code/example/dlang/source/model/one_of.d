import model;
import stream;
import std.conv;
import std.typecons : Nullable;

abstract class OneOf {
    abstract void writeTo(Stream writer) const;
    static OneOf readFrom(Stream reader) {
        switch (reader.readInt()) {
            case OptionOne.TAG:
                return OptionOne.readFrom(reader);
            case OptionTwo.TAG:
                return OptionTwo.readFrom(reader);
            default:
                throw new Exception("Unexpected tag value");
        }
    }

    static class OptionOne : OneOf {
        static const int TAG = 0;
        int[] value;
        this() {}
        this(int[] value) {
            this.value = value;
        }
        static OptionOne readFrom(Stream reader) {
            auto result = new OptionOne();
            result.value = new int[reader.readInt()];
            for (int i = 0; i < result.value.length; i++) {
                result.value[i] = reader.readInt();
            }
            return result;
        }
        override void writeTo(Stream writer) const {
            writer.write(TAG);
            writer.write(cast(int)(value.length));
            foreach (valueElement; value) {
                writer.write(valueElement);
            }
        }
        override string toString() const {
            return "OptionOne" ~ "(" ~
                to!string(value) ~
                ")";
        }
    }

    static class OptionTwo : OneOf {
        static const int TAG = 1;
        int value;
        this() {}
        this(int value) {
            this.value = value;
        }
        static OptionTwo readFrom(Stream reader) {
            auto result = new OptionTwo();
            result.value = reader.readInt();
            return result;
        }
        override void writeTo(Stream writer) const {
            writer.write(TAG);
            writer.write(value);
        }
        override string toString() const {
            return "OptionTwo" ~ "(" ~
                to!string(value) ~
                ")";
        }
    }
}
