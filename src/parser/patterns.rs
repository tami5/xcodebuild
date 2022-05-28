use super::util::define_pattern;
use lazy_regex::*;

define_pattern! {
    ident: Analyze,
    desc: "Analyze/AnalyzeShallow",
    captures: [ filepath, filename, target, project ],
    pattern: r"(?x)
      Analyze(?:Shallow)?\s
      # Filepath and filename
      ( ?P<filepath>.*/( ?P<filename>.*\.(?:mm|m|cc|cpp|c|cxx) ) )
      ( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        r"AnalyzeShallow /path/to/file.m normal x86_64 (in target 'MyTarget' from project 'MyProject')" =>
            |captures| {
                assert_eq!("/path/to/file.m", &captures["filepath"]);
                assert_eq!("file.m", &captures["filename"]);
                assert_eq!("MyTarget", &captures["target"]);
                assert_eq!("MyProject", &captures["project"]);
            },
        r"AnalyzeShallow /path/to/file.c" =>
            |captures| {
                assert_eq!("/path/to/file.c", &captures["filepath"]);
                assert_eq!("file.c", &captures["filename"]);
            },
        r"Analyze /path/to/file.mm" =>
            |captures| {
                assert_eq!("/path/to/file.mm", &captures["filepath"]);
                assert_eq!("file.mm", &captures["filename"]);
            }
    }
}

define_pattern! {
    ident: BuildTarget,
    desc: "BUILD TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sBUILD\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD TARGET ExampleTarget OF PROJECT ExampleProject WITH THE DEFAULT CONFIGURATION Local ===" =>
            |captures| {
                assert_eq!("ExampleTarget", &captures["target"]);
                assert_eq!("ExampleProject", &captures["project"]);
                assert_eq!("Local", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: AggregateTarget,
    desc: "BUILD AGGREGATE TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sBUILD\sAGGREGATE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== BUILD AGGREGATE TARGET Example Target Name OF PROJECT AggregateTarget WITH CONFIGURATION Debug ===" =>
            |captures| {
                assert_eq!("Example Target Name", &captures["target"]);
                assert_eq!("AggregateTarget", &captures["project"]);
                assert_eq!("Debug", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: AnalyzeTarget,
    desc: "ANALYZE TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sANALYZE\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== ANALYZE TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: ShellCommand,
    desc: "Shell commands like cd setenv under a compile step",
    captures: [ command,arguments ],
    pattern: r"\s{4}(?P<command>cd|setenv|(?:[\w/:\s\-.]+?/)?[\w\-]+)\s(?P<arguments>.*)$",
    tests: {
        "    cd /foo/bar/baz" =>
            |captures| {
                assert_eq!("cd", &captures["command"]);
                assert_eq!("/foo/bar/baz", &captures["arguments"]);
            }
    }
}

define_pattern! {
    ident: CleanRemove,
    desc: "CLEAN REMOVE",
    captures: [ filepath, filename ],
    pattern: r"(?x)Clean.Remove\sclean\s
      # filepath and filename
      ( ?P<filepath>.*/ ( ?P<filename>.*\.(?:build) ))",
    tests: {
        "Clean.Remove clean /path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build" =>
            |captures| {
                assert_eq!("/path/to/MyLibrary.build/Debug-iphonesimulator/MyLibraryTests.build", &captures["filepath"]);
                assert_eq!("MyLibraryTests.build", &captures["filename"]);
            }
    }
}

define_pattern! {
    ident: CleanTarget,
    desc: "CLEAN TARGET",
    captures: [ target, project, configuration ],
    pattern: r"={3}\sCLEAN\sTARGET\s(?P<target>.*)\sOF\sPROJECT\s(?P<project>.*)\sWITH.*CONFIGURATION\s(?P<configuration>.*)\s={3}",
    tests: {
        "=== CLEAN TARGET X OF PROJECT Y WITH THE DEFAULT CONFIGURATION Z ===" =>
            |captures| {
                assert_eq!("X", &captures["target"]);
                assert_eq!("Y", &captures["project"]);
                assert_eq!("Z", &captures["configuration"]);
            }
    }
}

define_pattern! {
    ident: CodeSign,
    desc: "CodeSign Phase",
    captures: [ filename, target, project ],
    pattern: r"CodeSign\s(:?.*/(?P<filename>.*\.(?:app)))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CodeSign path/to/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("DemoTarget.app", &captures["filename"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: Compile,
    desc: r"Compile(Swift|C|\w) Step",
    captures: [ type, filename, filepath, target, project ],
    pattern: r"(?x)
        # Compile <type>
        Compile(?P<type>[\w]+)\s.+?\s
        # <filepath>
        (?P<filepath>(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:m|mm|c|cc|cpp|cxx|swift)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileSwift normal arm64 /path/to/ToastView.swift (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Swift", &captures["type"]);
                assert_eq!("/path/to/ToastView.swift", &captures["filepath"]);
                assert_eq!("ToastView.swift", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CompileC /path/to/output/arm64/bridge.o /path/to/bridge.c normal arm64 c com.apple.compilers.llvm.clang.1_0.compiler (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("C", &captures["type"]);
                assert_eq!("/path/to/bridge.c", &captures["filepath"]);
                assert_eq!("bridge.c", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
            // "CompileAssetCatalog /output/Example.app /input/Assets.xcassets (in target 'Example' from project 'Example')" =>
            //     |captures| {
            //     assert_eq!("AssetCatalog", &captures["type"]);
            //     assert_eq!("/input/Assets.xcassets", &captures["filepath"]);
            //     assert_eq!("Assets.xcassets", &captures["filename"]);
            //     assert_eq!("Example", &captures["project"]);
            //     assert_eq!("Example", &captures["target"]);
            // }

    }
}

define_pattern! {
    ident: CompileCommand,
    desc: r"Clang and swiftc command",
    captures: [ command, arguments ],
    pattern: r"\s{4}(:?[^\s]+/(?P<command>\w+))\s(?P<arguments>.*)",
    tests: {
        "    /TOOLCHAIN_BIN/clang -target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o" =>
            |captures| {
                assert_eq!("clang", &captures["command"]);
                assert_eq!("-target arm64-apple-macos10.10 -r -isysroot /MACOS_SDK -L/BUILD_ROOT -L/MACOS_SDK/lib -o /BUILD_ROOT/file.o", &captures["arguments"]);
            },
        r"    /TOOLCHAIN_BIN/swiftc -incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT" =>
            |captures| {
                assert_eq!("swiftc", &captures["command"]);
                assert_eq!(r"-incremental -module-name Example -Onone -enable-batch-mode -enforce-exclusivity\=checked -working-directory /PROJECT_ROOT", &captures["arguments"]);
            }
            // NOTE: Won't match  /TOOLCHAIN_BIN/swift-frontend -frontend -c file.swift
    }
}

define_pattern! {
    ident: CompileXIB,
    desc: r"CompileXIB",
    captures: [ filename, filepath, project, target ],
    pattern: r"CompileXIB\s(?P<filepath>.*/(?P<filename>.*\.xib))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileXIB /path/to/MainMenu.xib (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/MainMenu.xib", &captures["filepath"]);
                assert_eq!("MainMenu.xib", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
}

define_pattern! {
    ident: CompileStoryboard,
    desc: r"CompileStoryboard",
    captures: [ filename, filepath, project, target ],
    pattern: r"CompileStoryboard\s(?P<filepath>.*/(?P<filename>[^/].*\.storyboard))(?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CompileStoryboard /path/to/LaunchScreen.storyboard (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/LaunchScreen.storyboard", &captures["filepath"]);
                assert_eq!("LaunchScreen.storyboard", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }
    }
}

define_pattern! {
    ident: CopyCommand,
    desc: r"CpResource|CpHeader|CopyStringsFile|CopyPlistFile",
    captures: [ type, filename, filepath, project, target ],
    pattern: r"(?x)
               (:?Cp|Copy)(?P<type>Resource|Header|PlistFile|StringsFile)\s.*\s
               (?P<filepath>.*/(?P<filename>.*\.(?:\w+)))
               (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "CpResource /output/EnWords.txt /path/to/EnWords.txt (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Resource", &captures["type"]);
                assert_eq!("/path/to/EnWords.txt", &captures["filepath"]);
                assert_eq!("EnWords.txt", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
        "CpHeader /output/file.h /path/to/file.h (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("Header", &captures["type"]);
                assert_eq!("/path/to/file.h", &captures["filepath"]);
                assert_eq!("file.h", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            },
         "CopyStringsFile /output/InfoPlist.strings path/to/en.lproj/InfoPlist.strings (in target 'Example' from project 'Example')" => |captures| {
                assert_eq!("StringsFile", &captures["type"]);
                assert_eq!("path/to/en.lproj/InfoPlist.strings", &captures["filepath"]);
                assert_eq!("InfoPlist.strings", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);

            }
    }
}

define_pattern! {
    ident: TestExecuted,
    desc: r"Executed number of tests",
    captures: [ tests_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    pattern: r"(?x)\s*Executed\s
        (?P<tests_count>\d+)\stest[s]?,\swith\s
        (?P<failed_tests_count>\d+)\sfailure[s]?\s
        \((?P<unexpected_test_count>\d+)\sunexpected\)\sin\s\d+\.\d{3}\s
        \((?P<total_exec_time>\d+\.\d{3})\)\sseconds",
    tests: {
        "     Executed 3 tests, with 1 failure (0 unexpected) in 0.258 (0.259) seconds" =>
            |captures| {
                assert_eq!("3", &captures["tests_count"]);
                assert_eq!("1", &captures["failed_tests_count"]);
                assert_eq!("0", &captures["unexpected_test_count"]);
                assert_eq!("0.259", &captures["total_exec_time"]);
            },
        "Executed 4 tests, with 0 failures (0 unexpected) in 0.003 (0.004) seconds" =>
            |captures| {
                assert_eq!("4", &captures["tests_count"]);
                assert_eq!("0", &captures["failed_tests_count"]);
                assert_eq!("0", &captures["unexpected_test_count"]);
                assert_eq!("0.004", &captures["total_exec_time"]);
            }
    }
}

define_pattern! {
    ident: TestExecutedWithSkipped,
    desc: r"Executed number of tests with skipped teats",
    captures: [ tests_count, skipped_test_count, failed_tests_count, unexpected_test_count, total_exec_time ],
    pattern: r"(?x)
        \s*Executed\s
        (?P<tests_count>\d+)\stest[s]?,\swith\s
        (?P<skipped_test_count>\d+)\stest[s]?\sskipped\sand\s
        (?P<failed_tests_count>\d+)\sfailure[s]?\s
        \((?P<unexpected_test_count>\d+)\sunexpected\)\sin\s\d+\.\d{3}\s
        \((?P<total_exec_time>\d+\.\d{3})\)\sseconds",
    tests: {
        "    Executed 56 tests, with 3 test skipped and 2 failures (1 unexpected) in 1.029 (1.029) seconds" =>
            |captures| {
                assert_eq!("56", &captures["tests_count"]);
                assert_eq!("3", &captures["skipped_test_count"]);
                assert_eq!("2", &captures["failed_tests_count"]);
                assert_eq!("1", &captures["unexpected_test_count"]);
                assert_eq!("1.029", &captures["total_exec_time"]);
            },
        "Executed 1 test, with 1 test skipped and 1 failure (1 unexpected) in 3.000 (3.000) seconds" =>
            |captures| {
                assert_eq!("1", &captures["tests_count"]);
                assert_eq!("1", &captures["skipped_test_count"]);
                assert_eq!("1", &captures["failed_tests_count"]);
                assert_eq!("1", &captures["unexpected_test_count"]);
                assert_eq!("3.000", &captures["total_exec_time"]);
            }
    }
}

define_pattern! {
    ident: KiwiFailingTest,
    desc: r"Kiwi Test failing",
    captures: [ filepath, suite, case, reason ],
    pattern: r"(?x)\s*
        (?P<filepath>.+:\d+):\serror:\s[\+\-]
        \[
          (?P<suite>.*)\s
          (?P<case>.*)
         \]\s:(?:\s'.*'\s\[FAILED\],)?\s
        (?P<reason>.*)",
    tests: {
        "/path/to/tests.m:49: error: -[TestSuite TestCase] : 'Iterators, times： iterates the exact number of times' [FAILED], expected subject to equal 4, got 5" =>
            |captures| {
                assert_eq!("/path/to/tests.m:49", &captures["filepath"]);
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("expected subject to equal 4, got 5", &captures["reason"]);
            }
    }
}

define_pattern! {
    ident: UIFailingTest,
    desc: r"UI Test failing",
    captures: [ filepath, reason ],
    pattern: r"\s*t = \s+\d+\.\d+s\s+Assertion Failure: (?P<filepath>.*:\d+): (?P<reason>.*)$",
    tests: {
        "t =    22.27s             Assertion Failure: <unknown>:0: UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'" =>
            |captures| {
                assert_eq!("<unknown>:0", &captures["filepath"]);
                assert_eq!("UI Testing Failure - Unable to find hit point for element Button 0x608001165880: {{74.0, -54.0}, {44.0, 38.0}}, label: 'Disconnect'", &captures["reason"]);
            }
    }
}

define_pattern! {
    ident: CoverageReportGeneration,
    desc: r"Coverage report generation",
    captures: [ filepath ],
    pattern: r"(?i)generated\s+coverage\s+report:\s+(?P<filepath>.+)",
    tests: {
        "Generated coverage report: /path/to/code coverage.xccovreport" =>
            |captures| {
                assert_eq!("/path/to/code coverage.xccovreport", &captures["filepath"]);
            }
    }
}

define_pattern! {
    ident: GenerateDsymFile,
    desc: r"GenerateDSYMFile",
    captures: [ filename, target, project ],
    pattern: r"(?x)
        GenerateDSYMFile\s/.*/
        (?P<filename>.*\.dSYM)\s/.*
        \((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)?",
    tests: {
        "GenerateDSYMFile /$BUILD/Release/DemoTarget.app.dSYM /$BUILD/Release/DemoTarget.app/Contents/MacOS/DemoTarget (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("DemoTarget.app.dSYM", &captures["filename"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: Linking,
    desc: r"Ld",
    captures: [ filename, target, project ],
    pattern: r"Ld\s(?P<filepath>.*/(?P<filename>\w+\.\w+)).*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)?",
    tests: {
        "Ld /path/to/file.o normal x86_64 (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/file.o", &captures["filepath"]);
                assert_eq!("file.o", &captures["filename"]);
                assert_eq!("Example", &captures["target"]);
                assert_eq!("Example", &captures["project"]);
            }
    }
}

// - TESTING ----------------------------------------------------------------------

define_pattern! {
    ident: TestSuiteStarted,
    desc: r"Test Suites Started",
    captures: [ name, time ],
    pattern: r"\s*Test Suite '(?:.*/)?(?P<name>.*[ox]ctest.*)' started at (?P<time>.*)",
    tests: {
        "Test Suite 'ObjectiveRecordTests.xctest' started at 2013-12-10 06:15:39 +0000" =>
            |captures| {
                assert_eq!("ObjectiveRecordTests.xctest", &captures["name"]);
                assert_eq!("2013-12-10 06:15:39 +0000", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: TestSuiteCompleted,
    desc: r"Test Suites Completed",
    captures: [ name, time ],
    pattern: r"\s*Test Suite '(?:.*/)?(?P<name>.*[ox]ctest.*)' (finished|passed|failed) at (?P<time>.*)\.",
    tests: {
        "Test Suite 'ObjectiveRecordTests.xctest' finished at 2013-12-10 06:15:42 +0000." =>
            |captures| {
                assert_eq!("ObjectiveRecordTests.xctest", &captures["name"]);
                assert_eq!("2013-12-10 06:15:42 +0000", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: TestCaseStarted,
    desc: r"Test Case Started",
    captures: [ suite, case],
    pattern: r"\s*Test Case '-\[(?P<suite>.*) (?P<case>.*)\]' started.$",
    tests: {
        "Test Case '-[viewUITests.vmtAboutWindow testConnectToDesktop]' started." =>
            |captures| {
                assert_eq!("viewUITests.vmtAboutWindow", &captures["suite"]);
                assert_eq!("testConnectToDesktop", &captures["case"]);
            }
    }
}

define_pattern! {
    ident: TestCasePassed,
    desc: r"Test Case Passed",
    captures: [ suite, case, time ],
    pattern: r"\s*Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\spassed\s\((?P<time>\d*\.\d{3})\sseconds\).",
    tests: {
        "Test Case '-[TestSuite TestCase]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.001", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: KiwiTestCasePending,
    desc: r"Kiwi test case pending",
    captures: [ suite, case ],
    pattern: r"Test Case\s'-\[(?P<suite>.*)\s(?P<case>.*)PENDING\]'\spassed",
    tests: {
        "Test Case '-[TestSuite TestCasePENDING]' passed (0.001 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
            }
    }
}

define_pattern! {
    ident: TestCaseMeasure,
    desc: r"Test case measuring",
    captures: [ suite, case, time ],
    pattern: r"[^:]*:[^:]*:\sTest Case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\smeasured\s\[Time,\sseconds\]\saverage:\s(?P<time>\d*\.\d{3})(.*){4}",
    tests: {
        r#"<unknown>:0: Test Case '-[TestSuite TestCase]' measured [Time, seconds] average: 0.013, relative standard deviation: 26.773%, values: [0.023838, 0.012034, ], performanceMetricID:com.apple.XCTPerformanceMetric_WallClockTime, baselineName: "", baselineAverage: , maxPercentRegression: 10.000%, maxPercentRelativeStandardDeviation: 10.000%, maxRegression: 0.100, maxStandardDeviation: 0.100"# =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("TestCase", &captures["case"]);
                assert_eq!("0.013", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestCasePassed,
    desc: r"Parallel TestCase passed",
    captures: [ suite, case, time, medium ],
    pattern: r"Test\s+case\s+'(?P<suite>.*)\.(?P<case>.*)\(\)'\s+passed\s+on\s+'(?P<medium>.*)'\s+\((?P<time>\d*\.(.*){3})\s+seconds\)",
    tests: {
        "Test case 'TestSuite.testCase()' passed on 'xctest (49438)' (0.131 seconds)" =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("0.131", &captures["time"]);
                assert_eq!("xctest (49438)", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestCaseAppKitPassed,
    desc: r"Parallel TestCase AppKit Passed",
    captures: [ suite, case, time, medium ],
    pattern: r"\s*Test case\s'-\[(?P<suite>.*)\s(?P<case>.*)\]'\spassed\son\s'(?P<medium>.*)'\s\((?P<time>\d*\.\d{3})\sseconds\)",
    tests: {
        "Test case '-[TestSuite testCase]' passed on 'xctest (49438)' (0.131 seconds)." =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("xctest (49438)", &captures["medium"]);
                assert_eq!("0.131", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestCaseFailed,
    desc: r"Parallel TestCase Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"Test\s+case\s+'(?P<suite>.*)\.(?P<case>.*)\(\)'\s+failed\s+on\s+'(?P<medium>.*)'\s+\((?P<time>\d*\.(.*){3})\s+seconds\)",
    tests: {
        "Test case 'TestSuite.testCase()' failed on 'iPhone 11' (7.158 seconds)" =>
            |captures| {
                assert_eq!("TestSuite", &captures["suite"]);
                assert_eq!("testCase", &captures["case"]);
                assert_eq!("iPhone 11", &captures["medium"]);
                assert_eq!("7.158", &captures["time"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestingStarted,
    desc: r"Parallel Testing Started",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing started on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestingPassed,
    desc: r"Parallel Testing Passed",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+passed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing passed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestingFailed,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"Testing\s+failed\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Testing failed on 'iPhone X'" =>
            |captures| {
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: ParallelTestFailed,
    desc: r"Parallel Testing Failed",
    captures: [ suite, case, time, medium ],
    pattern: r"(?i)\s*Test\s+Suite\s+'(?P<suite>.*)'\s+started\s+on\s+'(?P<medium>.*)'",
    tests: {
        "Test suite 'TestSuite (iOS).xctest' started on 'iPhone X'" =>
            |captures| {
                assert_eq!("TestSuite (iOS).xctest", &captures["suite"]);
                assert_eq!("iPhone X", &captures["medium"]);
            }
    }
}

define_pattern! {
    ident: PhaseScriptExecution,
    desc: r"PhaseScriptExecution",
    captures: [ name, target, project ],
    pattern: r"(?x)PhaseScriptExecution\s(?P<name>.*)\s/.*\.sh( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        "PhaseScriptExecution Format\\ Swift\\ Files /path/to/file.sh (in target 'DemoTarget' from project 'DemoProject')" =>
            |captures| {
                assert_eq!("Format\\ Swift\\ Files", &captures["name"]);
                assert_eq!("DemoTarget", &captures["target"]);
                assert_eq!("DemoProject", &captures["project"]);
            },
        "PhaseScriptExecution [CP]\\ Check\\ Pods\\ Manifest.lock /path/to/file.sh (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("[CP]\\ Check\\ Pods\\ Manifest.lock", &captures["name"]);
                assert_eq!("App", &captures["target"]);
                assert_eq!("App", &captures["project"]);
            }

    }
}

define_pattern! {
    ident: ProcessPCH,
    desc: r"ProcessPCH",
    captures: [ filename, target, project ],
    pattern: r"(?x)ProcessPCH(?:\+\+)?\s.*\s/.*/(?P<filename>.*.pch)( ?:\s.* \((?:in\starget\s      '(?P<target>.*)'\s  from\sproject\s   '(?P<project>.*)' )\)  ) ?",
    tests: {
        "ProcessPCH /path/to/file.pch.gch /path/to/file.pch normal x86_64 objective-c com.apple.compilers.llvm.clang.1_0.analyzer (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("file.pch", &captures["filename"]);
                assert_eq!("App", &captures["target"]);
                assert_eq!("App", &captures["project"]);
            }
    }
}

define_pattern! {
    ident: ProcessPCHCommand,
    desc: r"ProcessPchCommand",
    captures: [ ],
    pattern: r"\s*.*/usr/bin/clang\s.*\s\-c\s(.*.pch)\s.*\-o\s.*"
}

define_pattern! {
    ident: PbxCopy,
    desc: r"PBXCp",
    captures: [ filename, target, project ],
    pattern: r"(?x)PBXCp\s(?P<filepath>/.*)\s/.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\)",
    tests: {
        "PBXCp /path/to/header.h /path/to/output.h (in target 'App' from project 'App')" =>
            |captures| {
                assert_eq!("App" ,&captures["target"]);
                assert_eq!("App" ,&captures["project"]);
                assert_eq!("/path/to/header.h" ,&captures["filepath"]);
            }
    }
}

define_pattern! {
    ident: ProcessInfoPlistFile,
    desc: r"ProcessInfoPlistFile",
    captures: [ filename, filepath, target, project ],
    pattern: r"(?x)ProcessInfoPlistFile\s.*\s
        (?P<filepath>/(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:plist)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "ProcessInfoPlistFile /path/to/output/Info.plist /path/to/Info.plist (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/path/to/Info.plist", &captures["filepath"]);
                assert_eq!("Info.plist", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }


    }
}

define_pattern! {
    ident: CheckDependencies,
    desc: r"Check dependencies",
    captures: [],
    pattern: r"Check dependencies"
}

define_pattern! {
    ident: RestartingTests,
    desc: r"Test restarting",
    captures: [],
    pattern:r"Restarting after unexpected exit.+$"
}

define_pattern! {
    ident: CoverageDataGeneration,
    desc: r"Coverage Data Generation",
    captures: [],
    pattern:r"generating\s+coverage\s+data\.*"
}

define_pattern! {
    ident: PhaseSuccess,
    desc: r"Phase Success",
    captures: [],
    pattern: r"\*\*\s(.*)\sSUCCEEDED\s\*\*"
}

define_pattern! {
    ident: TestSuiteAllTestsPassed,
    desc: r"Test Suite All Tests Passed",
    captures: [],
    pattern: r"\s*Test Suite 'All tests' passed at"
}

define_pattern! {
    ident: TestSuiteAllTestsFailed,
    desc: r"Test Suite All Tests Passed",
    captures: [],
    pattern: r"\s*Test Suite 'All tests' failed at"
}

define_pattern! {
    ident: Touch,
    desc: r"Touch file",
    captures: [ filename, filepath, target, project ],
    pattern: r"(?x)Touch\s(?P<filepath>/(?:\.|[^\s])+/(?P<filename>(?:\.|[^\s])+\.(?:\w+)))
        (?:\s.*\((?:in\starget\s'(?P<target>.*)'\sfrom\sproject\s'(?P<project>.*)')\))?",
    tests: {
        "Touch /BUILD_ROOT/Example.app (in target 'Example' from project 'Example')" =>
            |captures| {
                assert_eq!("/BUILD_ROOT/Example.app", &captures["filepath"]);
                assert_eq!("Example.app", &captures["filename"]);
                assert_eq!("Example", &captures["project"]);
                assert_eq!("Example", &captures["target"]);
            }


    }
}

// - Warning ----------------------------------------------------------------------

define_pattern! {
    ident: CompileWarning,
    desc: r"Compile Warning",
    captures: [ location, filepath, message ],
    pattern: r"(?P<location>(?P<filepath>[^:]*):\d*:\d*):\swarning:\s(?P<message>.*)$",
    tests: {
        "/path/file.swift:64:69: warning: 'flatMap' is deprecated: Please use compactMap(_:) for the case where closure returns an optional value" =>
            |captures| {
                assert_eq!("/path/file.swift:64:69", &captures["location"]);
                assert_eq!("/path/file.swift", &captures["filepath"]);
                assert_eq!("'flatMap' is deprecated: Please use compactMap(_:) for the case where closure returns an optional value", &captures["message"]);
            }


    }
}

define_pattern! {
    ident: LdWarning,
    desc: r"Linking Warning",
    captures: [ prefix, message ],
    pattern: r"(ld:.*)warning: (?P<msg>.*)"
}

define_pattern! {
    ident: GenericWarning,
    desc: r"Generic Error (catch all)",
    captures: [ message ],
    pattern: r"warning:\s(?P<message>.*)$"
}

define_pattern! {
    ident: CodeSignWarning,
    desc: r"Sign warnning",
    captures: [ message ],
    pattern: r"(?P<message>.* will not be code signed because .*)$"
}

// - Error ------------------------------------------------------------------------

define_pattern! {
    ident: ClangError,
    desc: r"Clang Error",
    captures: [ message ],
    pattern: r"(?P<message>clang: error:.*)$",
    tests: {
        "clang: error: linker command failed with exit code 1 (use -v to see invocation)" =>
        |captures| {
            assert_eq!("clang: error: linker command failed with exit code 1 (use -v to see invocation)", &captures["message"])

        }
    }
}

define_pattern! {
    ident: CheckDependenciesError,
    desc: r"Check Dependencies error",
    captures: [ message ],
    pattern: r"(?P<message>Code\s?Sign error:.*|Code signing is required for product type .* in SDK .*|No profile matching .* found:.*|Provisioning profile .* doesn't .*|Swift is unavailable on .*|.?Use Legacy Swift Language Version.*)$"
}

define_pattern! {
    ident: ProvisioningProfileRequiredError,
    desc: r"General Check Depeds error",
    captures: [ message ],
    pattern: r"(.*requires a provisioning profile.*)$"
}

define_pattern! {
    ident: NoCertificateError,
    desc: r"General Check Depeds error",
    captures: [ message ],
    pattern: r"(?P<message>No certificate matching.*)$"
}

define_pattern! {
    ident: CompileError,
    desc: r"Compile Error",
    captures: [ message ],
    pattern: r"\s*(?P<location>(?P<filepath>[^:]*):\d*:\d*):\s(?:fatal\s)?error:\s(?P<message>.*)$",
    tests: {
        "/path/file.swift:64:69: error: cannot find 'input' in scope" =>
        |captures| {
            assert_eq!("/path/file.swift:64:69", &captures["location"]);
            assert_eq!("/path/file.swift", &captures["filepath"]);
            assert_eq!("cannot find 'input' in scope", &captures["message"]);
        }
    }
}

define_pattern! {
    ident: Cursor,
    desc: r"Cursor",
    captures: [ content ],
    pattern: r"(?P<content>[\s~]*\^[\s~]*)$"
}

define_pattern! {
    ident: FatalError,
    desc: r"Compile Error",
    captures: [ message ],
    pattern: r"(?P<message>fatal error:.*)$"
}

define_pattern! {
    ident: FileMissingError,
    desc: r"File missing Error",
    captures: [ message, filepath ],
    pattern: r"<unknown>:0:\s(?P<message>error:\s.*)\s'(?P<filepath>/.+/.*\..*)'$"
}

define_pattern! {
    ident: LdError,
    desc: r"Ld Error",
    captures: [ message ],
    pattern: r"(P<message>ld:.*)"
}

define_pattern! {
    ident: LinkerDuplicateSymbolsLocationError,
    desc: r"duplicate symbols location",
    captures: [ message ],
    pattern: r"\s+(?P<message>/.*\.o[\)]?)$"
}

define_pattern! {
    ident: LinkerDuplicateSymbolsError,
    desc: r"Linker Duplicate Symbols Error",
    captures: [ message ],
    pattern: r"(?P<message>duplicate symbol .*):$"
}

define_pattern! {
    ident: LinkerUndefinedSymbolsLocationError,
    desc: r"Linker Undefined Symbols Location Error",
    captures: [ message ],
    pattern: r"(P?<message>.* in .*\.o)$"
}

define_pattern! {
    ident: LinkerUndefinedSymbolsError,
    desc: r"Undefined symbols",
    captures: [ message ],
    pattern: r"(P?<message>.* in .*\.o)$"
}

define_pattern! {
    ident: PodsError,
    desc: r"Pods error",
    captures: [ message ],
    pattern: r"(P?<message>error:\s.*)"
}

define_pattern! {
    ident: SymbolReferencedFrom,
    desc: r"Symbol reference from error",
    captures: [ message ],
    pattern: "\\s+\"(?P<message>.*)\", referenced from:$"
}

define_pattern! {
    ident: ModuleIncludesError,
    desc: r"module includes error",
    captures: [ message ],
    pattern: r"<module-includes>:.*?:.*?:\s(?:fatal\s)?(P?<message>error:\s.*)$/"
}

define_pattern! {
    ident: UndefinedSymbolLocationError,
    desc: r"Undefined symol location",
    captures: [ message ],
    pattern: r".+ in (.+)\((.+)\.o\)$"
}

define_pattern! {
    ident: PackageGraphResolvingStart,
    desc: r"Package Graph Resolving Start",
    captures: [ message ],
    pattern: r"\s*(Resolve Package Graph)\s*$"
}

define_pattern! {
    ident: PackageGraphResolvingEnd,
    desc: r"Package Graph Resolving Ended",
    captures: [ message ],
    pattern: r"(Resolved source packages):$"
}

define_pattern! {
    ident: PackageGraphResolvedItem,
    desc: r"Package Graph Resolved Item",
    captures: [ message ],
    pattern: r"\s*([^\s:]+):\s([^ ]+)\s@\s(\d+\.\d+\.\d+)"
}
