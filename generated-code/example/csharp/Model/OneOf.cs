namespace TransGenTest.Model
{
    public abstract class OneOf
    {
        public abstract void WriteTo(System.IO.BinaryWriter writer);
        public static OneOf ReadFrom(System.IO.BinaryReader reader)
        {
            switch (reader.ReadInt32())
            {
                case OptionOne.TAG:
                    return OptionOne.ReadFrom(reader);
                case OptionTwo.TAG:
                    return OptionTwo.ReadFrom(reader);
                default:
                    throw new System.Exception("Unexpected tag value");
            }
        }

        public class OptionOne : OneOf
        {
            public const int TAG = 0;
            public int[] VecI32 { get; set; }
            public long LongInt { get; set; }
            public OptionOne() {}
            public OptionOne(int[] vecI32, long longInt)
            {
                this.VecI32 = vecI32;
                this.LongInt = longInt;
            }
            public static new OptionOne ReadFrom(System.IO.BinaryReader reader)
            {
                var result = new OptionOne();
                result.VecI32 = new int[reader.ReadInt32()];
                for (int i = 0; i < result.VecI32.Length; i++)
                {
                    result.VecI32[i] = reader.ReadInt32();
                }
                result.LongInt = reader.ReadInt64();
                return result;
            }
            public override void WriteTo(System.IO.BinaryWriter writer)
            {
                writer.Write(TAG);
                writer.Write(VecI32.Length);
                foreach (var VecI32Element in VecI32)
                {
                    writer.Write(VecI32Element);
                }
                writer.Write(LongInt);
            }
        }

        public class OptionTwo : OneOf
        {
            public const int TAG = 1;
            public int Value { get; set; }
            public OptionTwo() {}
            public OptionTwo(int value)
            {
                this.Value = value;
            }
            public static new OptionTwo ReadFrom(System.IO.BinaryReader reader)
            {
                var result = new OptionTwo();
                result.Value = reader.ReadInt32();
                return result;
            }
            public override void WriteTo(System.IO.BinaryWriter writer)
            {
                writer.Write(TAG);
                writer.Write(Value);
            }
        }
    }
}
