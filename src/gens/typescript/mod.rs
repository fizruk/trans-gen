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

fn type_name(schema: &Schema) -> String {
    match schema {
        Schema::Bool => "boolean".to_owned(),
        Schema::Int32 => "number".to_owned(),
        Schema::Int64 => "bigint".to_owned(),
        Schema::Float32 => "number".to_owned(),
        Schema::Float64 => "number".to_owned(),
        Schema::String => "string".to_owned(),
        Schema::Option(inner) => format!("{} | null", type_name(inner)),
        Schema::Struct {
            definition: Struct { name, .. },
            ..
        } => name.camel_case(conv),
        Schema::OneOf { base_name, .. } => base_name.camel_case(conv),
        Schema::Vec(inner) => format!("Array<{}>", type_name(inner)),
        Schema::Map(key_type, value_type) => {
            format!("Map<{}, {}>", type_name(key_type), type_name(value_type))
        }
        Schema::Enum { base_name, .. } => base_name.camel_case(conv),
    }
}

fn imports(schema: &Schema) -> String {
    let mut imports = BTreeSet::new();
    fn add_imports_struct(definition: &Struct, imports: &mut BTreeSet<Name>) {
        fn add_imports(schema: &Schema, imports: &mut BTreeSet<Name>) {
            match schema {
                Schema::Struct {
                    definition: Struct { name, .. },
                    ..
                }
                | Schema::OneOf {
                    base_name: name, ..
                }
                | Schema::Enum {
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
                Schema::Bool
                | Schema::Int32
                | Schema::Int64
                | Schema::Float32
                | Schema::Float64
                | Schema::String => {}
            }
        }
        for field in &definition.fields {
            add_imports(&field.schema, imports);
        }
    }
    match schema {
        Schema::Struct { definition, .. } => {
            add_imports_struct(definition, &mut imports);
        }
        Schema::OneOf { variants, .. } => {
            for variant in variants {
                add_imports_struct(variant, &mut imports);
            }
        }
        _ => {}
    }
    include_templing!("src/gens/typescript/imports.templing")
}

fn doc_comment(documentation: &Documentation) -> String {
    let mut result = String::new();
    result.push_str("/**\n");
    for line in documentation.get("en").unwrap().lines() {
        result.push_str(" * ");
        result.push_str(line);
        result.push('\n');
    }
    result.push_str(" */\n");
    result.trim().to_owned()
}

fn doc_read_from(name: &str) -> String {
    format!("/**\n * Read {} from input stream\n */", name)
}

fn doc_write_to(name: &str) -> String {
    format!("/**\n * Write {} to output stream\n */", name)
}

fn read_var(var: &str, schema: &Schema) -> String {
    include_templing!("src/gens/typescript/read_var.templing")
}

fn write_var(var: &str, schema: &Schema) -> String {
    include_templing!("src/gens/typescript/write_var.templing")
}

fn struct_impl(definition: &Struct, base: Option<(&Name, usize)>) -> String {
    include_templing!("src/gens/typescript/struct_impl.templing")
}

fn file_name(name: &Name) -> String {
    name.snake_case(conv).replace('_', "-")
}

impl Generator {
    fn add_only(&mut self, schema: &Schema) -> anyhow::Result<()> {
        match schema {
            Schema::Enum {
                namespace,
                documentation,
                base_name,
                variants,
            } => {
                writeln!(
                    self.index_file,
                    "import {{ {} }} from './{}';\nexport {{ {} }};",
                    base_name.camel_case(conv),
                    file_name(base_name),
                    base_name.camel_case(conv),
                )
                .unwrap();
                self.files.insert(
                    format!("src/model/{}.ts", file_name(base_name)),
                    include_templing!("src/gens/typescript/enum.templing"),
                );
            }
            Schema::Struct {
                namespace,
                definition,
            } => {
                writeln!(
                    self.index_file,
                    "import {{ {} }} from './{}';\nexport {{ {} }};",
                    definition.name.camel_case(conv),
                    file_name(&definition.name),
                    definition.name.camel_case(conv),
                )
                .unwrap();
                self.files.insert(
                    format!("src/model/{}.ts", file_name(&definition.name)),
                    include_templing!("src/gens/typescript/struct.templing"),
                );
            }
            Schema::OneOf {
                namespace,
                documentation,
                base_name,
                variants,
            } => {
                writeln!(
                    self.index_file,
                    "import {{ {} }} from './{}';\nexport {{ {} }};",
                    base_name.camel_case(conv),
                    file_name(base_name),
                    base_name.camel_case(conv),
                )
                .unwrap();
                self.files.insert(
                    format!("src/model/{}.ts", file_name(base_name)),
                    include_templing!("src/gens/typescript/oneof.templing"),
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
    const NAME: &'static str = "TypeScript";
    type Options = ();
    fn new(name: &str, version: &str, _: ()) -> Self {
        let project_name = Name::new(name.to_owned())
            .snake_case(conv)
            .replace('_', "-");
        let project_version = version;
        let mut files = HashMap::new();
        files.insert(
            "src/stream.ts".to_owned(),
            include_str!("stream.ts").to_owned(),
        );
        files.insert(
            "tsconfig.json".to_owned(),
            include_str!("tsconfig.json").to_owned(),
        );
        files.insert(
            "package.json".to_owned(),
            include_templing!("src/gens/typescript/package.json.templing").to_owned(),
        );
        Self {
            files,
            index_file: String::new(),
        }
    }
    fn generate(mut self, extra_files: Vec<File>) -> GenResult {
        self.files
            .insert("src/model/index.ts".to_owned(), self.index_file);
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
        command("npm").arg("install").current_dir(path).run()?;
        command("npm")
            .arg("run")
            .arg("build")
            .current_dir(path)
            .run()?;
        Ok(())
    }
    fn run_local(path: &Path) -> anyhow::Result<Command> {
        let mut command = command("node");
        command.arg("main.js").current_dir(path.join("build"));
        Ok(command)
    }
}

impl<D: Trans + PartialEq + Debug> TestableGenerator<testing::FileReadWrite<D>> for Generator {
    fn extra_files(test: &testing::FileReadWrite<D>) -> Vec<File> {
        let schema = Schema::of::<D>(&test.version);
        let schema: &Schema = &schema;
        vec![File {
            path: "src/main.ts".to_owned(),
            content: include_templing!("src/gens/typescript/file-read-write.ts.templing"),
        }]
    }
}

impl<D: Trans + PartialEq + Debug> TestableGenerator<testing::TcpReadWrite<D>> for Generator {
    fn extra_files(test: &testing::TcpReadWrite<D>) -> Vec<File> {
        let schema = Schema::of::<D>(&test.version);
        let schema: &Schema = &schema;
        vec![
            File {
                path: "src/tcp-stream.ts".to_owned(),
                content: include_str!("tcp-stream.ts").to_owned(),
            },
            File {
                path: "src/main.ts".to_owned(),
                content: include_templing!("src/gens/typescript/tcp-read-write.ts.templing"),
            },
        ]
    }
}
