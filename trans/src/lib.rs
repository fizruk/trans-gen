use byteorder::{ReadBytesExt, WriteBytesExt};

pub use trans_derive::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait Trans: Sized {
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()>;
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self>;
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_snake_case, unused_variables)]
        impl<$($name: Trans),*> Trans for ($($name,)*) {
            fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
                Ok(($(<$name as Trans>::read_from(reader)?,)*))
            }
            fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
                let ($($name,)*) = self;
                $($name.write_to(writer)?;)*
                Ok(())
            }
        }
    };
}

impl_for_tuple!();
impl_for_tuple!(A);
impl_for_tuple!(A, B);
impl_for_tuple!(A, B, C);
impl_for_tuple!(A, B, C, D);

impl Trans for bool {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let value = reader.read_u8()?;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bool value should be 0 or 1",
            )),
        }
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        writer.write_u8(if *self { 1 } else { 0 })
    }
}

impl Trans for usize {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(i32::read_from(reader)? as usize)
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        (*self as i32).write_to(writer)
    }
}

impl Trans for i32 {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        reader.read_i32::<byteorder::LittleEndian>()
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        writer.write_i32::<byteorder::LittleEndian>(*self)
    }
}

impl Trans for i64 {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        reader.read_i64::<byteorder::LittleEndian>()
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        writer.write_i64::<byteorder::LittleEndian>(*self)
    }
}

impl Trans for f32 {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        reader.read_f32::<byteorder::LittleEndian>()
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        writer.write_f32::<byteorder::LittleEndian>(*self)
    }
}

impl Trans for f64 {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        reader.read_f64::<byteorder::LittleEndian>()
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        writer.write_f64::<byteorder::LittleEndian>(*self)
    }
}

impl Trans for String {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let len = i32::read_from(reader)? as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        (self.len() as i32).write_to(writer)?;
        writer.write_all(self.as_bytes())
    }
}

impl<T: Trans> Trans for Option<T> {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let is_some = bool::read_from(reader)?;
        Ok(if is_some {
            Some(T::read_from(reader)?)
        } else {
            None
        })
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.is_some().write_to(writer)?;
        if let Some(value) = self {
            value.write_to(writer)?;
        }
        Ok(())
    }
}

impl<T: Trans> Trans for Vec<T> {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let len = usize::read_from(reader)?;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::read_from(reader)?);
        }
        Ok(result)
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.len().write_to(writer)?;
        for item in self {
            item.write_to(writer)?;
        }
        Ok(())
    }
}

impl<K: Trans + Eq + std::hash::Hash, V: Trans> Trans for std::collections::HashMap<K, V> {
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let len = usize::read_from(reader)?;
        let mut result = Self::with_capacity(len);
        for _ in 0..len {
            result.insert(K::read_from(reader)?, V::read_from(reader)?);
        }
        Ok(result)
    }
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.len().write_to(writer)?;
        for (key, value) in self {
            key.write_to(writer)?;
            value.write_to(writer)?;
        }
        Ok(())
    }
}