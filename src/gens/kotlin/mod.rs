use super::*;

fn conv(name: &str) -> String {
    name.replace("Bool", "Boolean")
        .replace("Int32", "Int")
        .replace("Int64", "Long")
        .replace("Float32", "Float")
        .replace("Float64", "Double")
}

pub struct Generator {
    files: HashMap<String, String>,
}

fn type_name(schema: &Schema) -> String {
    match schema {
        Schema::Bool => "Boolean".to_owned(),
        Schema::Int32 => "Int".to_owned(),
        Schema::Int64 => "Long".to_owned(),
        Schema::Float32 => "Float".to_owned(),
        Schema::Float64 => "Double".to_owned(),
        Schema::String => "String".to_owned(),
        Schema::Struct(Struct { name, .. })
        | Schema::OneOf {
            base_name: name, ..
        }
        | Schema::Enum {
            base_name: name, ..
        } => format!("model.{}", name.camel_case(conv)),
        Schema::Option(inner) => format!("{}?", type_name(inner)),
        Schema::Vec(inner) => format!("Array<{}>", type_name(inner)),
        Schema::Map(key, value) => format!("MutableMap<{}, {}>", type_name(key), type_name(value)),
    }
}

fn default_value(schema: &Schema) -> String {
    match schema {
        Schema::Bool => "false".to_owned(),
        Schema::Int32 => "0".to_owned(),
        Schema::Int64 => "0L".to_owned(),
        Schema::Float32 => "0.0f".to_owned(),
        Schema::Float64 => "0.0".to_owned(),
        Schema::Option(_) => "null".to_owned(),
        Schema::String
        | Schema::Struct(_)
        | Schema::OneOf { .. }
        | Schema::Enum { .. }
        | Schema::Vec(_)
        | Schema::Map(_, _) => unreachable!(),
    }
}

fn can_lateinit(schema: &Schema) -> bool {
    match schema {
        Schema::Bool
        | Schema::Int32
        | Schema::Int64
        | Schema::Float32
        | Schema::Float64
        | Schema::Option(_) => false,
        Schema::String
        | Schema::Struct(_)
        | Schema::OneOf { .. }
        | Schema::Enum { .. }
        | Schema::Vec(_)
        | Schema::Map(_, _) => true,
    }
}

fn index_var_name(index_var: &mut usize) -> String {
    let result = "ijk".chars().nth(*index_var).unwrap();
    *index_var += 1;
    result.to_string()
}

fn var_name(name: &str) -> &str {
    match name.rfind('.') {
        Some(index) => &name[(index + 1)..],
        None => name,
    }
}

fn write_struct(
    writer: &mut Writer,
    struc: &Struct,
    base: Option<(&Name, usize)>,
) -> std::fmt::Result {
    // Class
    if let Some((base_name, _)) = base {
        writeln!(
            writer,
            "class {} : {} {{",
            struc.name.camel_case(conv),
            base_name.camel_case(conv)
        )?;
    } else {
        writeln!(writer, "class {} {{", struc.name.camel_case(conv))?;
    }
    writer.inc_ident();

    // Fields
    for field in &struc.fields {
        writeln!(
            writer,
            "{}var {}: {}{}",
            if can_lateinit(&field.schema) {
                "lateinit "
            } else {
                ""
            },
            field.name.mixed_case(conv),
            type_name(&field.schema),
            if can_lateinit(&field.schema) {
                String::new()
            } else {
                format!(" = {}", default_value(&field.schema))
            }
        )?;
    }

    // Constructor
    writeln!(writer, "constructor() {{}}")?;
    if !struc.fields.is_empty() {
        write!(writer, "constructor(")?;
        for (index, field) in struc.fields.iter().enumerate() {
            if index > 0 {
                write!(writer, ", ")?;
            }
            write!(
                writer,
                "{}: {}",
                field.name.mixed_case(conv),
                type_name(&field.schema),
            )?;
        }
        writeln!(writer, ") {{")?;
        for field in &struc.fields {
            writeln!(
                writer,
                "    this.{} = {}",
                field.name.mixed_case(conv),
                field.name.mixed_case(conv)
            )?;
        }
        writeln!(writer, "}}")?;
    }

    // Reading
    writeln!(writer, "companion object {{")?;
    writer.inc_ident();
    if let Some((_, tag)) = base {
        writeln!(writer, "val TAG = {}", tag)?;
    }
    writeln!(writer, "@Throws(java.io.IOException::class)")?;
    writeln!(
        writer,
        "fun readFrom(stream: java.io.InputStream): {} {{",
        struc.name.camel_case(conv),
    )?;
    writer.inc_ident();
    writeln!(writer, "val result = {}()", struc.name.camel_case(conv),)?;
    for field in &struc.fields {
        fn assign(
            writer: &mut Writer,
            to: &str,
            schema: &Schema,
            index_var: &mut usize,
        ) -> std::fmt::Result {
            match schema {
                Schema::Bool => {
                    writeln!(writer, "{} = StreamUtil.readBoolean(stream)", to)?;
                }
                Schema::Int32 => {
                    writeln!(writer, "{} = StreamUtil.readInt(stream)", to)?;
                }
                Schema::Int64 => {
                    writeln!(writer, "{} = StreamUtil.readLong(stream)", to)?;
                }
                Schema::Float32 => {
                    writeln!(writer, "{} = StreamUtil.readFloat(stream)", to)?;
                }
                Schema::Float64 => {
                    writeln!(writer, "{} = StreamUtil.readDouble(stream)", to)?;
                }
                Schema::String => {
                    writeln!(writer, "{} = StreamUtil.readString(stream)", to)?;
                }
                Schema::Struct(Struct { name, .. })
                | Schema::OneOf {
                    base_name: name, ..
                } => {
                    writeln!(
                        writer,
                        "{} = model.{}.readFrom(stream)",
                        to,
                        name.camel_case(conv)
                    )?;
                }
                Schema::Option(inner) => {
                    writeln!(writer, "if (StreamUtil.readBoolean(stream)) {{")?;
                    writer.inc_ident();
                    assign(writer, to, inner, index_var)?;
                    writer.dec_ident();
                    writeln!(writer, "}} else {{")?;
                    writeln!(writer, "    {} = null", to)?;
                    writeln!(writer, "}}")?;
                }
                Schema::Vec(inner) => {
                    writeln!(writer, "{} = Array(StreamUtil.readInt(stream), {{", to)?;
                    writer.inc_ident();
                    writeln!(writer, "var {}Value: {}", var_name(to), type_name(inner))?;
                    assign(writer, &format!("{}Value", var_name(to)), inner, index_var)?;
                    writeln!(writer, "{}Value", var_name(to))?;
                    writer.dec_ident();
                    writeln!(writer, "}})")?;
                }
                Schema::Map(key_type, value_type) => {
                    let to_size = format!("{}Size", var_name(to));
                    writeln!(writer, "val {} = StreamUtil.readInt(stream)", to_size)?;
                    writeln!(writer, "{} = mutableMapOf()", to)?;
                    let index_var_name = index_var_name(index_var);
                    writeln!(writer, "for ({} in 0 until {}) {{", index_var_name, to_size)?;
                    writer.inc_ident();
                    writeln!(writer, "var {}Key: {}", var_name(to), type_name(key_type))?;
                    assign(writer, &format!("{}Key", var_name(to)), key_type, index_var)?;
                    writeln!(
                        writer,
                        "var {}Value: {}",
                        var_name(to),
                        type_name(value_type)
                    )?;
                    assign(
                        writer,
                        &format!("{}Value", var_name(to)),
                        value_type,
                        index_var,
                    )?;
                    writeln!(
                        writer,
                        "{}.put({}Key, {}Value)",
                        to,
                        var_name(to),
                        var_name(to)
                    )?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Enum {
                    documentation: _,
                    base_name,
                    variants,
                } => {
                    writeln!(writer, "when (StreamUtil.readInt(stream)) {{")?;
                    for (tag, variant) in variants.iter().enumerate() {
                        write!(writer, "{} ->", tag)?;
                        writeln!(
                            writer,
                            "{} = model.{}.{}",
                            to,
                            base_name.camel_case(conv),
                            variant.name.shouty_snake_case(conv)
                        )?;
                    }
                    writeln!(
                        writer,
                        "else -> throw java.io.IOException(\"Unexpected tag value\")"
                    )?;
                    writeln!(writer, "}}")?;
                }
            }
            Ok(())
        }
        assign(
            writer,
            &format!("result.{}", field.name.mixed_case(conv)),
            &field.schema,
            &mut 0,
        )?;
    }
    writeln!(writer, "return result")?;
    writer.dec_ident();
    writeln!(writer, "}}")?;
    writer.dec_ident();
    writeln!(writer, "}}")?;

    // Writing
    writeln!(writer, "@Throws(java.io.IOException::class)")?;
    writeln!(
        writer,
        "{}fun writeTo(stream: java.io.OutputStream) {{",
        if base.is_some() { "override " } else { "" }
    )?;
    writer.inc_ident();
    if base.is_some() {
        writeln!(writer, "StreamUtil.writeInt(stream, TAG)")?;
    }
    if let Some(magic) = struc.magic {
        writeln!(writer, "StreamUtil.writeInt(stream, {})", magic)?;
    }
    for field in &struc.fields {
        fn write(writer: &mut Writer, value: &str, schema: &Schema) -> std::fmt::Result {
            match schema {
                Schema::Bool => {
                    writeln!(writer, "StreamUtil.writeBoolean(stream, {})", value)?;
                }
                Schema::Int32 => {
                    writeln!(writer, "StreamUtil.writeInt(stream, {})", value)?;
                }
                Schema::Int64 => {
                    writeln!(writer, "StreamUtil.writeLong(stream, {})", value)?;
                }
                Schema::Float32 => {
                    writeln!(writer, "StreamUtil.writeFloat(stream, {})", value)?;
                }
                Schema::Float64 => {
                    writeln!(writer, "StreamUtil.writeDouble(stream, {})", value)?;
                }
                Schema::String => {
                    writeln!(writer, "StreamUtil.writeString(stream, {})", value)?;
                }
                Schema::Struct(_) | Schema::OneOf { .. } => {
                    writeln!(writer, "{}.writeTo(stream)", value)?;
                }
                Schema::Option(inner) => {
                    writeln!(writer, "val {} = {};", var_name(value), value)?;
                    writeln!(writer, "if ({} == null) {{", var_name(value))?;
                    writeln!(writer, "    StreamUtil.writeBoolean(stream, false)")?;
                    writeln!(writer, "}} else {{")?;
                    writer.inc_ident();
                    writeln!(writer, "StreamUtil.writeBoolean(stream, true)")?;
                    write(writer, &var_name(value), inner)?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Vec(inner) => {
                    writeln!(writer, "StreamUtil.writeInt(stream, {}.size)", value)?;
                    writeln!(writer, "for ({}Element in {}) {{", var_name(value), value)?;
                    writer.inc_ident();
                    write(writer, &format!("{}Element", var_name(value)), inner)?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Map(key_type, value_type) => {
                    writeln!(writer, "StreamUtil.writeInt(stream, {}.size)", value)?;
                    writeln!(writer, "for ({}Entry in {}) {{", var_name(value), value,)?;
                    writer.inc_ident();
                    write(writer, &format!("{}Entry.key", var_name(value)), key_type)?;
                    write(
                        writer,
                        &format!("{}Entry.value", var_name(value)),
                        value_type,
                    )?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Enum { .. } => {
                    writeln!(writer, "StreamUtil.writeInt(stream, {}.tag)", value)?;
                }
            }
            Ok(())
        }
        write(writer, &field.name.mixed_case(conv), &field.schema)?;
    }
    writer.dec_ident();
    writeln!(writer, "}}")?;
    writer.dec_ident();
    writeln!(writer, "}}")?;
    Ok(())
}

impl crate::Generator for Generator {
    const NAME: &'static str = "Kotlin";
    type Options = ();
    fn new(name: &str, _version: &str, _: ()) -> Self {
        let project_name = Name::new(name.to_owned())
            .snake_case(conv)
            .replace('_', "-");
        let project_name = &project_name;
        let mut files = HashMap::new();
        files.insert(
            "pom.xml".to_owned(),
            include_templing!("src/gens/kotlin/pom.xml.templing"),
        );
        files.insert(
            "src/main/kotlin/util/StreamUtil.kt".to_owned(),
            include_str!("StreamUtil.kt").to_owned(),
        );
        Self { files }
    }
    fn generate(mut self, extra_files: Vec<File>) -> GenResult {
        for file in extra_files {
            self.files.insert(file.path, file.content);
        }
        self.files.into()
    }
    fn add_only(&mut self, schema: &Schema) {
        match schema {
            Schema::Enum {
                documentation: _,
                base_name,
                variants,
            } => {
                let file_name = format!("src/main/kotlin/model/{}.kt", base_name.camel_case(conv));
                let mut writer = Writer::new();
                writeln!(writer, "package model").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "import util.StreamUtil").unwrap();
                writeln!(writer).unwrap();
                writeln!(
                    writer,
                    "enum class {} private constructor(var tag: Int) {{",
                    base_name.camel_case(conv)
                )
                .unwrap();
                writer.inc_ident();
                for (index, variant) in variants.iter().enumerate() {
                    writeln!(
                        writer,
                        "{}({}){}",
                        variant.name.shouty_snake_case(conv),
                        index,
                        if index + 1 < variants.len() { "," } else { "" }
                    )
                    .unwrap();
                }
                writer.dec_ident();
                writeln!(writer, "}}").unwrap();
                self.files.insert(file_name, writer.get());
            }
            Schema::Struct(struc) => {
                let file_name = format!("src/main/kotlin/model/{}.kt", struc.name.camel_case(conv));
                let mut writer = Writer::new();
                writeln!(writer, "package model").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "import util.StreamUtil").unwrap();
                writeln!(writer).unwrap();
                write_struct(&mut writer, struc, None).unwrap();
                self.files.insert(file_name, writer.get());
            }
            Schema::OneOf {
                documentation: _,
                base_name,
                variants,
            } => {
                let file_name = format!("src/main/kotlin/model/{}.kt", base_name.camel_case(conv));
                let mut writer = Writer::new();
                writeln!(writer, "package model").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "import util.StreamUtil").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "abstract class {} {{", base_name.camel_case(conv)).unwrap();
                {
                    writer.inc_ident();
                    writeln!(writer, "@Throws(java.io.IOException::class)").unwrap();
                    writeln!(writer, "abstract fun writeTo(stream: java.io.OutputStream)").unwrap();
                    writeln!(writer, "companion object {{").unwrap();
                    writer.inc_ident();
                    writeln!(writer, "@Throws(java.io.IOException::class)").unwrap();
                    writeln!(
                        writer,
                        "fun readFrom(stream: java.io.InputStream): {} {{",
                        base_name.camel_case(conv)
                    )
                    .unwrap();
                    {
                        writer.inc_ident();
                        writeln!(writer, "when (StreamUtil.readInt(stream)) {{").unwrap();
                        writer.inc_ident();
                        for variant in variants {
                            write!(writer, "{}.TAG -> ", variant.name.camel_case(conv)).unwrap();
                            writeln!(
                                writer,
                                "return {}.readFrom(stream)",
                                variant.name.camel_case(conv)
                            )
                            .unwrap();
                        }
                        writeln!(
                            writer,
                            "else -> throw java.io.IOException(\"Unexpected tag value\")"
                        )
                        .unwrap();
                        writer.dec_ident();
                        writeln!(writer, "}}").unwrap();
                        writer.dec_ident();
                    }
                    writeln!(writer, "}}").unwrap();
                    writer.dec_ident();
                    writeln!(writer, "}}").unwrap();
                    for (tag, variant) in variants.iter().enumerate() {
                        writeln!(writer).unwrap();
                        write_struct(&mut writer, variant, Some((base_name, tag))).unwrap();
                    }
                    writer.dec_ident();
                }
                writeln!(writer, "}}").unwrap();
                self.files.insert(file_name, writer.get());
            }
            Schema::Bool
            | Schema::Int32
            | Schema::Int64
            | Schema::Float32
            | Schema::Float64
            | Schema::String
            | Schema::Option(_)
            | Schema::Vec(_)
            | Schema::Map(_, _) => {}
        }
    }
}

impl RunnableGenerator for Generator {
    fn build_local(path: &Path) -> anyhow::Result<()> {
        command("mvn")
            .arg("package")
            .arg("--batch-mode")
            .current_dir(path)
            .run()
    }
    fn run_local(path: &Path) -> anyhow::Result<Command> {
        fn project_name(path: &Path) -> anyhow::Result<String> {
            let pom =
                std::fs::read_to_string(path.join("pom.xml")).context("Failed to read pom.xml")?;
            for line in pom.lines() {
                let line = line.trim();
                if let Some(line) = line.strip_prefix("<name>") {
                    if let Some(line) = line.strip_suffix("</name>") {
                        return Ok(line.trim().to_owned());
                    }
                }
            }
            anyhow::bail!("Failed to determine project name")
        }
        let mut command = command("java");
        command
            .arg("-jar")
            .arg(format!(
                "target/{}-jar-with-dependencies.jar",
                project_name(path)?,
            ))
            .current_dir(path);
        Ok(command)
    }
}

impl testing::FileReadWrite for Generator {
    fn extra_files(schema: &Schema) -> Vec<File> {
        vec![File {
            path: "src/main/kotlin/Runner.kt".to_owned(),
            content: include_templing!("src/gens/kotlin/FileReadWrite.kt.templing"),
        }]
    }
}
