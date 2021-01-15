use super::*;

fn conv(name: &str) -> String {
    name.replace("Int32", "Int")
        .replace("Int64", "Long")
        .replace("Float32", "Float")
        .replace("Float64", "Double")
}

pub struct Generator {
    files: HashMap<String, String>,
    model_include: String,
}

fn type_name(schema: &Schema) -> String {
    match schema {
        Schema::Bool => "bool".to_owned(),
        Schema::Int32 => "int".to_owned(),
        Schema::Int64 => "long long".to_owned(),
        Schema::Float32 => "float".to_owned(),
        Schema::Float64 => "double".to_owned(),
        Schema::String => "std::string".to_owned(),
        Schema::OneOf {
            base_name: name, ..
        } => format!("std::shared_ptr<{}>", name.camel_case(conv)),
        Schema::Struct(Struct { name, .. })
        | Schema::Enum {
            base_name: name, ..
        } => format!("{}", name.camel_case(conv)),
        Schema::Option(inner) => format!("std::shared_ptr<{}>", type_name(inner)),
        Schema::Vec(inner) => format!("std::vector<{}>", type_name(inner)),
        Schema::Map(key, value) => format!(
            "std::unordered_map<{}, {}>",
            type_name(key),
            type_name(value)
        ),
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

fn write_includes(writer: &mut Writer, schema: &Schema, current: bool) -> std::fmt::Result {
    let mut includes = vec!["<string>".to_string(), "\"../Stream.hpp\"".to_string()];
    collect_includes(&mut includes, schema, current);
    includes.sort();
    includes.dedup();
    for include in includes {
        writeln!(writer, "#include {}", include)?;
    }
    Ok(())
}

fn collect_includes(result: &mut Vec<String>, schema: &Schema, current: bool) {
    if current {
        match schema {
            Schema::Bool
            | Schema::Int32
            | Schema::Int64
            | Schema::Float32
            | Schema::Float64
            | Schema::String => {}
            Schema::Option(_) => {
                result.push("<memory>".to_string());
            }
            Schema::Map(_, _) => {
                result.push("<unordered_map>".to_string());
            }
            Schema::Vec(_) => {
                result.push("<vector>".to_string());
            }
            Schema::Struct(Struct { name, .. })
            | Schema::OneOf {
                base_name: name, ..
            }
            | Schema::Enum {
                base_name: name, ..
            } => {
                result.push("<stdexcept>".to_string());
                result.push(format!("\"{}.hpp\"", name.camel_case(conv)));
            }
        }
    }
    match schema {
        Schema::Bool
        | Schema::Int32
        | Schema::Int64
        | Schema::Float32
        | Schema::Float64
        | Schema::String
        | Schema::Enum { .. } => {}
        Schema::Option(inner) => {
            collect_includes(result, inner, true);
        }
        Schema::Map(key_type, value_type) => {
            collect_includes(result, key_type, true);
            collect_includes(result, value_type, true);
        }
        Schema::Vec(inner) => {
            collect_includes(result, inner, true);
        }
        Schema::Struct(Struct { fields, .. }) => {
            for field in fields {
                collect_includes(result, &field.schema, true);
            }
        }
        Schema::OneOf { variants, .. } => {
            for variant in variants {
                for field in &variant.fields {
                    collect_includes(result, &field.schema, true);
                }
            }
        }
    }
}

fn write_struct_def(
    writer: &mut Writer,
    schema: &Schema,
    struc: &Struct,
    base: Option<(&Name, usize)>,
) -> std::fmt::Result {
    let full_name = if let Some((base_name, _)) = base {
        format!(
            "{}::{}",
            base_name.camel_case(conv),
            struc.name.camel_case(conv)
        )
    } else {
        struc.name.camel_case(conv)
    };

    // Class
    if let Some((base_name, _)) = base {
        writeln!(
            writer,
            "class {}::{} : public {} {{",
            base_name.camel_case(conv),
            struc.name.camel_case(conv),
            base_name.camel_case(conv),
        )?;
    } else {
        writeln!(writer, "class {} {{", struc.name.camel_case(conv))?;
    }
    writer.inc_ident();
    if let Some((_, tag)) = base {
        writer.dec_ident();
        writeln!(writer, "public:")?;
        writer.inc_ident();
        writeln!(writer, "static const int TAG = {};", tag)?;
    }

    // Fields
    writer.dec_ident();
    writeln!(writer, "public:")?;
    writer.inc_ident();
    for field in &struc.fields {
        writeln!(
            writer,
            "{} {};",
            type_name(&field.schema),
            field.name.mixed_case(conv)
        )?;
    }

    // Constructor
    writeln!(writer, "{}();", struc.name.camel_case(conv))?;
    if !struc.fields.is_empty() {
        write!(writer, "{}(", struc.name.camel_case(conv))?;
        for (index, field) in struc.fields.iter().enumerate() {
            if index > 0 {
                write!(writer, ", ")?;
            }
            write!(
                writer,
                "{} {}",
                type_name(&field.schema),
                field.name.mixed_case(conv)
            )?;
        }
        writeln!(writer, ");")?;
    }

    // Read/write
    writeln!(
        writer,
        "static {} readFrom(InputStream& stream);",
        struc.name.camel_case(conv)
    )?;
    writeln!(
        writer,
        "void writeTo(OutputStream& stream) const{};",
        if base.is_some() { " override" } else { "" }
    )?;

    // Eq
    if schema.hashable() {
        writeln!(
            writer,
            "bool operator ==(const {}& other) const;",
            struc.name.camel_case(conv)
        )?;
    }

    writer.dec_ident();
    writeln!(writer, "}};").unwrap();

    // Hash
    if schema.hashable() {
        writeln!(writer, "namespace std {{")?;
        writer.inc_ident();
        writeln!(writer, "template<>")?;
        writeln!(writer, "struct hash<{}> {{", full_name)?;
        writeln!(
            writer,
            "    size_t operator ()(const {}& value) const;",
            full_name
        )?;
        writeln!(writer, "}};")?;
        writer.dec_ident();
        writeln!(writer, "}}")?;
    }

    Ok(())
}

fn write_struct_impl(
    writer: &mut Writer,
    schema: &Schema,
    struc: &Struct,
    base: Option<(&Name, usize)>,
) -> std::fmt::Result {
    let full_name = if let Some((base_name, _)) = base {
        format!(
            "{}::{}",
            base_name.camel_case(conv),
            struc.name.camel_case(conv)
        )
    } else {
        struc.name.camel_case(conv)
    };

    // Constructor
    writeln!(
        writer,
        "{}::{}() {{ }}",
        full_name,
        struc.name.camel_case(conv)
    )?;
    if !struc.fields.is_empty() {
        write!(writer, "{}::{}(", full_name, struc.name.camel_case(conv))?;
        for (index, field) in struc.fields.iter().enumerate() {
            if index > 0 {
                write!(writer, ", ")?;
            }
            write!(
                writer,
                "{} {}",
                type_name(&field.schema),
                field.name.mixed_case(conv),
            )?;
        }
        write!(writer, ") : ")?;
        for (index, field) in struc.fields.iter().enumerate() {
            write!(
                writer,
                "{}({})",
                field.name.mixed_case(conv),
                field.name.mixed_case(conv),
            )?;
            if index + 1 < struc.fields.len() {
                write!(writer, ", ")?;
            } else {
                writeln!(writer, " {{ }}")?;
            }
        }
    }

    // Read
    writeln!(
        writer,
        "{} {}::readFrom(InputStream& stream) {{",
        full_name, full_name,
    )?;
    writer.inc_ident();
    writeln!(writer, "{} result;", full_name)?;
    for field in &struc.fields {
        fn assign(
            writer: &mut Writer,
            to: &str,
            schema: &Schema,
            index_var: &mut usize,
        ) -> std::fmt::Result {
            match schema {
                Schema::Bool => {
                    writeln!(writer, "{} = stream.readBool();", to)?;
                }
                Schema::Int32 => {
                    writeln!(writer, "{} = stream.readInt();", to)?;
                }
                Schema::Int64 => {
                    writeln!(writer, "{} = stream.readLongLong();", to)?;
                }
                Schema::Float32 => {
                    writeln!(writer, "{} = stream.readFloat();", to)?;
                }
                Schema::Float64 => {
                    writeln!(writer, "{} = stream.readDouble();", to)?;
                }
                Schema::String => {
                    writeln!(writer, "{} = stream.readString();", to)?;
                }
                Schema::Struct(Struct { name, .. })
                | Schema::OneOf {
                    base_name: name, ..
                } => {
                    writeln!(
                        writer,
                        "{} = {}::readFrom(stream);",
                        to,
                        name.camel_case(conv)
                    )?;
                }
                Schema::Option(inner) => {
                    writeln!(writer, "if (stream.readBool()) {{")?;
                    writer.inc_ident();
                    writeln!(
                        writer,
                        "{} = std::shared_ptr<{}>(new {}());",
                        to,
                        type_name(inner),
                        type_name(inner)
                    )?;
                    assign(writer, &format!("*{}", to), inner, index_var)?;
                    writer.dec_ident();
                    writeln!(writer, "}} else {{")?;
                    writeln!(
                        writer,
                        "    {} = std::shared_ptr<{}>();",
                        to,
                        type_name(inner)
                    )?;
                    writeln!(writer, "}}")?;
                }
                Schema::Vec(inner) => {
                    writeln!(
                        writer,
                        "{} = std::vector<{}>(stream.readInt());",
                        to,
                        type_name(inner),
                    )?;
                    let index_var_name = index_var_name(index_var);
                    writeln!(
                        writer,
                        "for (size_t {} = 0; {} < {}.size(); {}++) {{",
                        index_var_name, index_var_name, to, index_var_name
                    )?;
                    writer.inc_ident();
                    assign(
                        writer,
                        &format!("{}[{}]", to, index_var_name),
                        inner,
                        index_var,
                    )?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Map(key_type, value_type) => {
                    let to_size = format!("{}Size", var_name(to));
                    writeln!(writer, "size_t {} = stream.readInt();", to_size)?;
                    writeln!(
                        writer,
                        "{} = std::unordered_map<{}, {}>();",
                        to,
                        type_name(key_type),
                        type_name(value_type)
                    )?;
                    writeln!(writer, "{}.reserve({});", to, to_size)?;
                    let index_var_name = index_var_name(index_var);
                    writeln!(
                        writer,
                        "for (size_t {} = 0; {} < {}; {}++) {{",
                        index_var_name, index_var_name, to_size, index_var_name
                    )?;
                    writer.inc_ident();
                    writeln!(writer, "{} {}Key;", type_name(key_type), var_name(to))?;
                    assign(writer, &format!("{}Key", var_name(to)), key_type, index_var)?;
                    writeln!(writer, "{} {}Value;", type_name(value_type), var_name(to))?;
                    assign(
                        writer,
                        &format!("{}Value", var_name(to)),
                        value_type,
                        index_var,
                    )?;
                    writeln!(
                        writer,
                        "{}.emplace(std::make_pair({}Key, {}Value));",
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
                    writeln!(writer, "switch (stream.readInt()) {{")?;
                    for (tag, variant) in variants.iter().enumerate() {
                        writeln!(writer, "case {}:", tag)?;
                        writeln!(
                            writer,
                            "    {} = {}::{};",
                            to,
                            base_name.camel_case(conv),
                            variant.name.shouty_snake_case(conv)
                        )?;
                        writeln!(writer, "    break;")?;
                    }
                    writeln!(writer, "default:")?;
                    writeln!(
                        writer,
                        "    throw std::runtime_error(\"Unexpected tag value\");"
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
    writeln!(writer, "return result;")?;
    writer.dec_ident();
    writeln!(writer, "}}")?;

    // Writing
    writeln!(
        writer,
        "void {}::writeTo(OutputStream& stream) const {{",
        full_name,
    )?;
    writer.inc_ident();
    if base.is_some() {
        writeln!(writer, "stream.write(TAG);")?;
    }
    if let Some(magic) = struc.magic {
        writeln!(writer, "stream.write({});", magic)?;
    }
    for field in &struc.fields {
        fn write(writer: &mut Writer, value: &str, schema: &Schema) -> std::fmt::Result {
            match schema {
                Schema::Bool => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::Int32 => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::Int64 => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::Float32 => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::Float64 => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::String => {
                    writeln!(writer, "stream.write({});", value)?;
                }
                Schema::Struct(_) => {
                    writeln!(writer, "{}.writeTo(stream);", value)?;
                }
                Schema::OneOf { .. } => {
                    writeln!(writer, "{}->writeTo(stream);", value)?;
                }
                Schema::Option(inner) => {
                    writeln!(writer, "if ({}) {{", value)?;
                    writer.inc_ident();
                    writeln!(writer, "stream.write(true);")?;
                    write(writer, &format!("(*{})", value), inner)?;
                    writer.dec_ident();
                    writeln!(writer, "}} else {{")?;
                    writeln!(writer, "    stream.write(false);")?;
                    writeln!(writer, "}}")?;
                }
                Schema::Vec(inner) => {
                    writeln!(writer, "stream.write((int)({}.size()));", value)?;
                    writeln!(
                        writer,
                        "for (const {}& {}Element : {}) {{",
                        type_name(inner),
                        var_name(value),
                        value
                    )?;
                    writer.inc_ident();
                    write(writer, &format!("{}Element", var_name(value)), inner)?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Map(key_type, value_type) => {
                    writeln!(writer, "stream.write((int)({}.size()));", value)?;
                    writeln!(
                        writer,
                        "for (const auto& {}Entry : {}) {{",
                        var_name(value),
                        value
                    )?;
                    writer.inc_ident();
                    write(writer, &format!("{}Entry.first", var_name(value)), key_type)?;
                    write(
                        writer,
                        &format!("{}Entry.second", var_name(value)),
                        value_type,
                    )?;
                    writer.dec_ident();
                    writeln!(writer, "}}")?;
                }
                Schema::Enum { .. } => {
                    writeln!(writer, "stream.write((int)({}));", value)?;
                }
            }
            Ok(())
        }
        write(writer, &field.name.mixed_case(conv), &field.schema)?;
    }
    writer.dec_ident();
    writeln!(writer, "}}")?;

    // Eq
    if schema.hashable() {
        writeln!(
            writer,
            "bool {}::operator ==(const {}& other) const {{",
            full_name, full_name,
        )?;
        write!(writer, "    return ")?;
        for (index, field) in struc.fields.iter().enumerate() {
            if index > 0 {
                write!(writer, " && ")?;
            }
            write!(
                writer,
                "{} == other.{}",
                field.name.mixed_case(conv),
                field.name.mixed_case(conv),
            )?;
        }
        writeln!(writer, ";")?;
        writeln!(writer, "}}")?;
    }

    // Hash
    if schema.hashable() {
        writeln!(
            writer,
            "size_t std::hash<{}>::operator ()(const {}& value) const {{",
            full_name, full_name,
        )?;
        writer.inc_ident();
        writeln!(writer, "size_t result = 0;")?;
        for field in &struc.fields {
            writeln!(
                writer,
                "result ^= std::hash<{}>{{}}(value.{}) + 0x9e3779b9 + (result<<6) + (result>>2);",
                type_name(&field.schema),
                field.name.mixed_case(conv),
            )?;
        }
        writeln!(writer, "return result;")?;
        writer.dec_ident();
        writeln!(writer, "}}")?;
    }

    Ok(())
}

impl crate::Generator for Generator {
    const NAME: &'static str = "C++";
    type Options = ();
    fn new(name: &str, _version: &str, _: ()) -> Self {
        let project_name = name;

        let mut files = HashMap::new();
        files.insert(
            "Stream.hpp".to_owned(),
            include_str!("Stream.hpp").to_owned(),
        );
        files.insert(
            "Stream.cpp".to_owned(),
            include_str!("Stream.cpp").to_owned(),
        );
        files.insert(
            "CMakeLists.txt".to_owned(),
            include_templing!("src/gens/cpp/CMakeLists.txt.templing"),
        );
        Self {
            files,
            model_include: "#ifndef _MODEL_HPP_\n#define _MODEL_HPP_\n\n".to_owned(),
        }
    }
    fn generate(self, extra_files: Vec<File>) -> GenResult {
        let Self {
            mut files,
            mut model_include,
        } = self;
        model_include.push_str("\n#endif\n");
        files.insert("model/Model.hpp".to_owned(), model_include.to_owned());
        for file in extra_files {
            files.insert(file.path, file.content);
        }
        files.into()
    }
    fn add_only(&mut self, schema: &Schema) {
        match schema {
            Schema::Enum {
                documentation: _,
                base_name,
                variants,
            } => {
                let file_name = format!("model/{}.hpp", base_name.camel_case(conv));
                self.model_include.push_str(&format!(
                    "#include \"{}.hpp\"\n",
                    base_name.camel_case(conv)
                ));
                let mut writer = Writer::new();
                writeln!(
                    writer,
                    "#ifndef _MODEL_{}_HPP_",
                    base_name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(
                    writer,
                    "#define _MODEL_{}_HPP_",
                    base_name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "#include \"../Stream.hpp\"").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "enum {} {{", base_name.camel_case(conv)).unwrap();
                writer.inc_ident();
                for (index, variant) in variants.iter().enumerate() {
                    writeln!(
                        writer,
                        "{} = {}{}",
                        variant.name.shouty_snake_case(conv),
                        index,
                        if index + 1 < variants.len() { "," } else { "" }
                    )
                    .unwrap();
                }
                writer.dec_ident();
                writeln!(writer, "}};").unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "#endif").unwrap();
                self.files.insert(file_name, writer.get());
            }
            Schema::Struct(struc) => {
                let file_name = format!("model/{}.hpp", struc.name.camel_case(conv));
                self.model_include.push_str(&format!(
                    "#include \"{}.hpp\"\n",
                    struc.name.camel_case(conv)
                ));
                let mut writer = Writer::new();
                writeln!(
                    writer,
                    "#ifndef _MODEL_{}_HPP_",
                    struc.name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(
                    writer,
                    "#define _MODEL_{}_HPP_",
                    struc.name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(writer).unwrap();
                write_includes(&mut writer, schema, false).unwrap();
                writeln!(writer).unwrap();
                write_struct_def(&mut writer, schema, struc, None).unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "#endif").unwrap();
                self.files.insert(file_name, writer.get());

                let file_name = format!("model/{}.cpp", struc.name.camel_case(conv));
                let mut writer = Writer::new();
                writeln!(writer, "#include \"{}.hpp\"", struc.name.camel_case(conv)).unwrap();
                writeln!(writer).unwrap();
                write_struct_impl(&mut writer, schema, struc, None).unwrap();
                self.files.insert(file_name, writer.get());
            }
            Schema::OneOf {
                documentation: _,
                base_name,
                variants,
            } => {
                let file_name = format!("model/{}.hpp", base_name.camel_case(conv));
                self.model_include.push_str(&format!(
                    "#include \"{}.hpp\"\n",
                    base_name.camel_case(conv)
                ));
                let mut writer = Writer::new();
                writeln!(
                    writer,
                    "#ifndef _MODEL_{}_HPP_",
                    base_name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(
                    writer,
                    "#define _MODEL_{}_HPP_",
                    base_name.shouty_snake_case(conv)
                )
                .unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "#include <memory>").unwrap();
                write_includes(&mut writer, schema, false).unwrap();
                writeln!(writer).unwrap();
                writeln!(writer, "class {} {{", base_name.camel_case(conv)).unwrap();
                writeln!(writer, "public:").unwrap();
                writer.inc_ident();
                for variant in variants {
                    writeln!(writer, "class {};", variant.name.camel_case(conv)).unwrap();
                }
                writeln!(writer).unwrap();
                writeln!(
                    writer,
                    "static std::shared_ptr<{}> readFrom(InputStream& stream);",
                    base_name.camel_case(conv)
                )
                .unwrap();
                writeln!(
                    writer,
                    "virtual void writeTo(OutputStream& stream) const = 0;",
                )
                .unwrap();
                writer.dec_ident();
                writeln!(writer, "}};").unwrap();
                for (tag, variant) in variants.iter().enumerate() {
                    writeln!(writer).unwrap();
                    write_struct_def(&mut writer, schema, variant, Some((base_name, tag))).unwrap();
                }
                writeln!(writer).unwrap();
                writeln!(writer, "#endif").unwrap();
                self.files.insert(file_name, writer.get());

                let file_name = format!("model/{}.cpp", base_name.camel_case(conv));
                let mut writer = Writer::new();
                writeln!(writer, "#include \"{}.hpp\"", base_name.camel_case(conv)).unwrap();
                writeln!(writer, "#include <stdexcept>").unwrap();
                for (tag, variant) in variants.iter().enumerate() {
                    writeln!(writer).unwrap();
                    write_struct_impl(&mut writer, schema, variant, Some((base_name, tag)))
                        .unwrap();
                }

                // Reading
                writeln!(
                    writer,
                    "std::shared_ptr<{}> {}::readFrom(InputStream& stream) {{",
                    base_name.camel_case(conv),
                    base_name.camel_case(conv),
                )
                .unwrap();
                writer.inc_ident();
                writeln!(writer, "switch (stream.readInt()) {{").unwrap();
                for (tag, variant) in variants.iter().enumerate() {
                    writeln!(writer, "case {}:", tag).unwrap();
                    let variant_name = format!(
                        "{}::{}",
                        base_name.camel_case(conv),
                        variant.name.camel_case(conv)
                    );
                    writeln!(
                        writer,
                        "    return std::shared_ptr<{}>(new {}({}::readFrom(stream)));",
                        variant_name, variant_name, variant_name,
                    )
                    .unwrap();
                }
                writeln!(writer, "default:").unwrap();
                writeln!(
                    writer,
                    "    throw std::runtime_error(\"Unexpected tag value\");"
                )
                .unwrap();
                writeln!(writer, "}}").unwrap();
                writer.dec_ident();
                writeln!(writer, "}};").unwrap();

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
        let standard: &str = "17";
        command("cmake")
            .current_dir(path)
            .arg(format!("-DCMAKE_CXX_STANDARD={}", standard))
            .arg("-DCMAKE_BUILD_TYPE=RELEASE")
            .arg("-DCMAKE_VERBOSE_MAKEFILE=ON")
            .arg(".")
            .run()?;
        command("cmake")
            .current_dir(path)
            .arg("--build")
            .arg(".")
            .arg("--config")
            .arg("Release")
            .run()?;
        Ok(())
    }
    fn run_local(path: &Path) -> anyhow::Result<Command> {
        let exe_dir = PathBuf::from(if cfg!(windows) { "Release" } else { "." });
        fn executable(path: &Path) -> anyhow::Result<String> {
            for line in std::fs::read_to_string(path.join("CMakeLists.txt"))?.lines() {
                if let Some(args) = line.strip_prefix("add_executable(") {
                    match args.split_whitespace().next() {
                        Some(executable) => return Ok(executable.to_owned()),
                        None => anyhow::bail!("Failed to parse executable()"),
                    }
                }
            }
            anyhow::bail!("Failed to determine executable");
        };
        let executable = executable(path)?;
        let mut command = command(
            exe_dir
                .join(format!(
                    "{}{}",
                    executable,
                    if cfg!(windows) { ".exe" } else { "" }
                ))
                .to_str()
                .unwrap(),
        );
        command.current_dir(path);
        Ok(command)
    }
}

impl testing::FileReadWrite for Generator {
    fn extra_files(schema: &Schema) -> Vec<File> {
        vec![File {
            path: "main.cpp".to_owned(),
            content: include_templing!("src/gens/cpp/file_read_write.cpp.templing"),
        }]
    }
}
