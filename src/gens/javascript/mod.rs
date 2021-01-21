use super::*;

fn conv(name: &str) -> String {
    name.replace("Int32", "Int")
        .replace("Int64", "Long")
        .replace("Float32", "Float")
        .replace("Float64", "Double")
}

pub struct Generator {
    files: HashMap<String, String>,
    index_file: String,
}

fn imports(schema: &Schema) -> String {
    let mut imports = BTreeSet::new();
    fn add_imports_struct(struc: &Struct, imports: &mut BTreeSet<Name>) {
        fn add_imports(schema: &Schema, imports: &mut BTreeSet<Name>) {
            match schema {
                Schema::Struct(Struct { name, .. })
                | Schema::OneOf {
                    base_name: name, ..
                } => {
                    imports.insert(name.clone());
                }
                Schema::Option(inner) => {
                    add_imports(inner, imports);
                }
                Schema::Vec(inner) => {
                    add_imports(inner, imports);
                }
                Schema::Map(key_type, value_type) => {
                    add_imports(key_type, imports);
                    add_imports(value_type, imports);
                }
                Schema::Enum { .. } => {}
                Schema::Bool
                | Schema::Int32
                | Schema::Int64
                | Schema::Float32
                | Schema::Float64
                | Schema::String => {}
            }
        }
        for field in &struc.fields {
            add_imports(&field.schema, imports);
        }
    }
    match schema {
        Schema::Struct(struc) => {
            add_imports_struct(struc, &mut imports);
        }
        Schema::OneOf { variants, .. } => {
            for variant in variants {
                add_imports_struct(variant, &mut imports);
            }
        }
        _ => {}
    }
    include_templing!("src/gens/javascript/imports.templing")
}

fn read_var(var: &str, schema: &Schema) -> String {
    include_templing!("src/gens/javascript/read_var.templing")
}

fn write_var(var: &str, schema: &Schema) -> String {
    include_templing!("src/gens/javascript/write_var.templing")
}

fn struct_impl(struc: &Struct, base: Option<(&Name, usize)>) -> String {
    include_templing!("src/gens/javascript/struct_impl.templing")
}

fn file_name(name: &Name) -> String {
    name.snake_case(conv).replace('_', "-")
}

impl Generator {
    fn add_only(&mut self, schema: &Schema) -> anyhow::Result<()> {
        match schema {
            Schema::Enum {
                documentation: _,
                base_name,
                variants,
            } => {
                writeln!(
                    self.index_file,
                    "module.exports.{} = require('./{}');",
                    base_name.camel_case(conv),
                    file_name(base_name),
                )
                .unwrap();
                self.files.insert(
                    format!("model/{}.js", file_name(base_name)),
                    include_templing!("src/gens/javascript/enum.templing"),
                );
            }
            Schema::Struct(struc) => {
                writeln!(
                    self.index_file,
                    "module.exports.{} = require('./{}');",
                    struc.name.camel_case(conv),
                    file_name(&struc.name),
                )
                .unwrap();
                self.files.insert(
                    format!("model/{}.js", file_name(&struc.name)),
                    include_templing!("src/gens/javascript/struct.templing"),
                );
            }
            Schema::OneOf {
                documentation: _,
                base_name,
                variants,
            } => {
                writeln!(
                    self.index_file,
                    "module.exports.{} = require('./{}');",
                    base_name.camel_case(conv),
                    file_name(base_name),
                )
                .unwrap();
                self.files.insert(
                    format!("model/{}.js", file_name(base_name)),
                    include_templing!("src/gens/javascript/oneof.templing"),
                );
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
        Ok(())
    }
}

impl crate::Generator for Generator {
    const NAME: &'static str = "JavaScript";
    type Options = ();
    fn new(_name: &str, _version: &str, _: ()) -> Self {
        let mut files = HashMap::new();
        files.insert(
            "stream-wrapper.js".to_owned(),
            include_str!("stream-wrapper.js").to_owned(),
        );
        Self {
            files,
            index_file: String::new(),
        }
    }
    fn generate(mut self, extra_files: Vec<File>) -> GenResult {
        self.files
            .insert("model/index.js".to_owned(), self.index_file);
        for file in extra_files {
            self.files.insert(file.path, file.content);
        }
        self.files.into()
    }
    fn add_only(&mut self, schema: &Schema) {
        self.add_only(schema).unwrap();
    }
}

impl RunnableGenerator for Generator {
    fn build_local(path: &Path) -> anyhow::Result<()> {
        Ok(())
    }
    fn run_local(path: &Path) -> anyhow::Result<Command> {
        let mut command = command("node");
        command.arg("main.js").current_dir(path);
        Ok(command)
    }
}

impl testing::FileReadWrite for Generator {
    fn extra_files(schema: &Schema) -> Vec<File> {
        fn type_name(schema: &Schema) -> String {
            match schema {
                Schema::Struct(Struct { name, .. })
                | Schema::OneOf {
                    base_name: name, ..
                } => name.camel_case(conv),
                _ => unreachable!(),
            }
        }
        vec![File {
            path: "main.js".to_owned(),
            content: include_templing!("src/gens/javascript/file-read-write.js.templing"),
        }]
    }
}