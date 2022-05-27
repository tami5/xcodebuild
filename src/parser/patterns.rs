use lazy_regex::*;

/**
    Analyze/AnalyzeShallow Command capture groups:

    - $file_path: Analyzed file path;
    - $file_name: Analyzed file name;
    - $target: Target Name;
    - $project: Project Name;
*/
pub static ANALYZE: Lazy<Regex> = lazy_regex! {
    r"(?x)
      Analyze(?:Shallow)?\s

      # File path and file name
      (
          # File Path
          ?P<file_path>.*/

          # File Name
          (?P<file_name>.*\.(?:mm|m|cc|cpp|c|cxx))
      )

      # Optional Whitespace
      (?:\s.*\(
          (?:
           # Target Name
           in\starget\s      '(?P<target>.*)'\s
           # Project Name
           from\sproject\s   '(?P<project>.*)'
           )
      \))?
      "
};

#[test]
fn test_analyze() {
    let text = r#"AnalyzeShallow /path/to/file.m normal x86_64 (in target 'MyTarget' from project 'MyProject')"#;
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.m", &captures["file_path"]);
    assert_eq!("file.m", &captures["file_name"]);
    assert_eq!("MyTarget", &captures["target"]);
    assert_eq!("MyProject", &captures["project"]);

    let text = r#"AnalyzeShallow /path/to/file.c"#;
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.c", &captures["file_path"]);
    assert_eq!("file.c", &captures["file_name"]);

    let text = "Analyze /path/to/file.mm";
    let captures = ANALYZE.captures(text).unwrap();
    assert_eq!("/path/to/file.mm", &captures["file_path"]);
    assert_eq!("file.mm", &captures["file_name"]);
}

/**
    BUILD TARGET captured groups:

    - $target = Target Name;
    - $project = Project Name;
    - $configuration = configuration
*/
pub static BUILD_TARGET: Lazy<Regex> = lazy_regex! {
    r"(?x)===\sBUILD\sTARGET\s
      # Target
      (?P<target>.*)
      # Project
      \sOF\sPROJECT\s(?P<project>.*)
      # Configuration
      \sWITH.*CONFIGURATION\s(?P<configuration>.*)\s===
     "
};

#[test]
fn test_build_target() {
    let text =
        "=== BUILD TARGET ExampleTarget OF PROJECT ExampleProject WITH THE DEFAULT CONFIGURATION Local ===";
    let captures = BUILD_TARGET.captures(text).unwrap();
    assert_eq!("ExampleTarget", &captures["target"]);
    assert_eq!("ExampleProject", &captures["project"]);
    assert_eq!("Local", &captures["configuration"]);
}

/**
    BUILD AGGREGATE TARGET captured groups:

    - $target = Target Name;
    - $project = Project Name;
    - $configuration = configuration
*/
pub static AGGREGATE_TARGET: Lazy<Regex> = lazy_regex! {
    r"(?x)===\sBUILD\sAGGREGATE\sTARGET\s
      # Target
      (?P<target>.*)
      # Project
      \sOF\sPROJECT\s(?P<project>.*)
      # Configuration
      \sWITH.*CONFIGURATION\s(?P<configuration>.*)\s===
     "
};

#[test]
fn test_aggregate_target() {
    let text =
        "=== BUILD AGGREGATE TARGET Example Target Name OF PROJECT AggregateTarget WITH CONFIGURATION Debug ===";
    let captures = AGGREGATE_TARGET.captures(text).unwrap();
    assert_eq!("Example Target Name", &captures["target"]);
    assert_eq!("AggregateTarget", &captures["project"]);
    assert_eq!("Debug", &captures["configuration"]);
}

/**
    ANALYZE TARGET captured groups:

    - $target = Target Name;
    - $project = Project Name;
    - $configuration = configuration
*/
pub static ANALYZE_TARGET: Lazy<Regex> = lazy_regex! {
    r"(?x)===\sANALYZE\sTARGET\s
      # Target
      (?P<target>.*)
      # Project
      \sOF\sPROJECT\s(?P<project>.*)
      # Configuration
      \sWITH.*CONFIGURATION\s(?P<configuration>.*)\s===
     "
};

#[test]
fn test_analyze_target() {
    let text = "=== ANALYZE TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===";
    let captures = ANALYZE_TARGET.captures(text).unwrap();
    assert_eq!("X", &captures["target"]);
    assert_eq!("Y", &captures["project"]);
    assert_eq!("Z", &captures["configuration"]);
}

/// Dependencies Check
pub static CHECK_DEPENDENCIES: Lazy<Regex> = lazy_regex!(r"Check dependencies");

/**
    shell command captured groups:

    - $command = Command Name;
    - $arguments = Arguments;
*/
pub static SHELL_COMMAND: Lazy<Regex> = lazy_regex! {
    r"(?x)\s{4}(?P<command>cd|setenv|(?:[\w/:\s\-.]+?/)?[\w\-]+)\s(?P<arguments>.*)$
     "
};

#[test]
fn test_shell_command() {
    let text = "    cd /foo/bar/baz";
    let captures = SHELL_COMMAND.captures(text).unwrap();
    assert_eq!("cd", &captures["command"]);
    assert_eq!("/foo/bar/baz", &captures["arguments"]);
}

/**
    CLEAN REMOVE pattern

    - $file_path = Cleanred file target;
    - $file_name = Cleanred File name;
*/
pub static CLEAN_REMOVE: Lazy<Regex> = lazy_regex! {
    r"(?x)Clean.Remove\sclean\s
      # File path and file name
      (
          # File Path
          ?P<file_path>.*/

          # File Name
          (?P<file_name>.*\.(?:build))
      )
"};

#[test]
fn test_clean_remove() {
    let text =
        "Clean.Remove clean /path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build";
    let captures = CLEAN_REMOVE.captures(text).unwrap();
    assert_eq!(
        "/path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build",
        &captures["file_path"]
    );
    assert_eq!("MyLibraryTests.build", &captures["file_name"]);
}

/**
    CLEAN TARGET captured groups:

    - $target = Target Name;
    - $project = Project Name;
    - $configuration = configuration
*/
pub static CLEAN_TARGET: Lazy<Regex> = lazy_regex! {
    r"(?x)===\sCLEAN\sTARGET\s
      # Target
      (?P<target>.*)
      # Project
      \sOF\sPROJECT\s(?P<project>.*)
      # Configuration
      \sWITH.*CONFIGURATION\s(?P<configuration>.*)\s===
     "
};

#[test]
fn test_clean_target() {
    let text = "=== CLEAN TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===";
    let captures = CLEAN_TARGET.captures(text).unwrap();
    assert_eq!("X", &captures["target"]);
    assert_eq!("Y", &captures["project"]);
    assert_eq!("Z", &captures["configuration"]);
}