use crate::parser::{get_commands_and_compile_errors, Step};

use super::super::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Clang compilation step
#[derive(Debug)]
pub struct CompileC {
    pub compiler: String,
    pub description: Description,
    pub output_path: PathBuf,
    pub path: PathBuf,
    pub arch: String,
    pub lang: String,
    pub command: String,
}

#[async_trait]
impl ParsableFromStream for CompileC {
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        let output_path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CompileC".into(), "output_path".into()))?;

        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CompileC".into(), "path".into()))?;

        chunks.next();

        let arch = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("CompileC".into(), "path".into()))?;

        let lang = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("CompileC".into(), "path".into()))?;

        let compiler = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("CompileC".into(), "path".into()))?;

        let description = Description::from_line(line)?;

        let (command, errors) = get_commands_and_compile_errors(stream).await;
        steps.extend(errors);

        let mut results = vec![Step::CompileC(Self {
            compiler,
            description,
            output_path,
            path,
            arch,
            lang,
            command,
        })];

        results.extend(steps);

        results.pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        CompileC,
       r#"CompileC path/to/output/bridge.o path/to/input/bridge.c normal arm64 c com.apple.compilers.llvm.clang.1_0.compiler (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    export LANG\=en_US.US-ASCII
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang ...

    "# 
    };
    if let Step::CompileC(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);

        assert_eq!("arm64", &step.arch);
        assert_eq!("c", &step.lang);
        assert_eq!(PathBuf::from("path/to/input/bridge.c"), step.path);
        assert_eq!(PathBuf::from("path/to/output/bridge.o"), step.output_path);
    } else {
        panic!("No script execution {steps:#?}")
    }
}

impl Display for CompileC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Compiling `{}`",
            self.description,
            self.path.file_name().unwrap().to_str().unwrap()
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn fmt() {
    let data = CompileC {
        compiler: "com.apple.compilers.llvm.clang.1_0.compiler".into(),
        description: Description {
            project: "DemoProject".into(),
            target: "DemoTarget".into(),
        },
        output_path: "".into(),
        path: PathBuf::from("/path/to/file.c"),
        arch: "arm".into(),
        lang: "c".into(),
        command: "".into(),
    };

    assert_eq!(
        "[DemoProject.DemoTarget] Compiling    `file.c`",
        &format!("{}", data),
    );
}
