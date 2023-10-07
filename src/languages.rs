use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Language {
    pub key: String,
    pub label: String,
}

pub fn get_languages() -> HashMap<String, Language> {
    let mut langs = HashMap::new();
    
    langs.insert(String::from("abap"), Language{ key: String::from("abap"), label: String::from("abap")} );
    langs.insert(String::from("abnf"), Language{ key: String::from("abnf"), label: String::from("abnf")} );
    langs.insert(String::from("actionscript"), Language{ key: String::from("actionscript"), label: String::from("actionscript")} );
    langs.insert(String::from("ada"), Language{ key: String::from("ada"), label: String::from("ada")} );
    langs.insert(String::from("agda"), Language{ key: String::from("agda"), label: String::from("agda")} );
    langs.insert(String::from("al"), Language{ key: String::from("al"), label: String::from("al")} );
    langs.insert(String::from("antlr4"), Language{ key: String::from("antlr4"), label: String::from("antlr4")} );
    langs.insert(String::from("apacheconf"), Language{ key: String::from("apacheconf"), label: String::from("apacheconf")} );
    langs.insert(String::from("apex"), Language{ key: String::from("apex"), label: String::from("apex")} );
    langs.insert(String::from("apl"), Language{ key: String::from("apl"), label: String::from("apl")} );
    langs.insert(String::from("applescript"), Language{ key: String::from("applescript"), label: String::from("applescript")} );
    langs.insert(String::from("aql"), Language{ key: String::from("aql"), label: String::from("aql")} );
    langs.insert(String::from("arduino"), Language{ key: String::from("arduino"), label: String::from("arduino")} );
    langs.insert(String::from("arff"), Language{ key: String::from("arff"), label: String::from("arff")} );
    langs.insert(String::from("asciidoc"), Language{ key: String::from("asciidoc"), label: String::from("asciidoc")} );
    langs.insert(String::from("asm6502"), Language{ key: String::from("asm6502"), label: String::from("asm6502")} );
    langs.insert(String::from("asmatmel"), Language{ key: String::from("asmatmel"), label: String::from("asmatmel")} );
    langs.insert(String::from("aspnet"), Language{ key: String::from("aspnet"), label: String::from("aspnet")} );
    langs.insert(String::from("autohotkey"), Language{ key: String::from("autohotkey"), label: String::from("autohotkey")} );
    langs.insert(String::from("autoit"), Language{ key: String::from("autoit"), label: String::from("autoit")} );
    langs.insert(String::from("avisynth"), Language{ key: String::from("avisynth"), label: String::from("avisynth")} );
    langs.insert(String::from("avroIdl"), Language{ key: String::from("avroIdl"), label: String::from("avroIdl")} );
    langs.insert(String::from("bash"), Language{ key: String::from("bash"), label: String::from("bash")} );
    langs.insert(String::from("basic"), Language{ key: String::from("basic"), label: String::from("basic")} );
    langs.insert(String::from("batch"), Language{ key: String::from("batch"), label: String::from("batch")} );
    langs.insert(String::from("bbcode"), Language{ key: String::from("bbcode"), label: String::from("bbcode")} );
    langs.insert(String::from("bicep"), Language{ key: String::from("bicep"), label: String::from("bicep")} );
    langs.insert(String::from("birb"), Language{ key: String::from("birb"), label: String::from("birb")} );
    langs.insert(String::from("bison"), Language{ key: String::from("bison"), label: String::from("bison")} );
    langs.insert(String::from("bnf"), Language{ key: String::from("bnf"), label: String::from("bnf")} );
    langs.insert(String::from("brainfuck"), Language{ key: String::from("brainfuck"), label: String::from("brainfuck")} );
    langs.insert(String::from("brightscript"), Language{ key: String::from("brightscript"), label: String::from("brightscript")} );
    langs.insert(String::from("bro"), Language{ key: String::from("bro"), label: String::from("bro")} );
    langs.insert(String::from("bsl"), Language{ key: String::from("bsl"), label: String::from("bsl")} );
    langs.insert(String::from("c"), Language{ key: String::from("c"), label: String::from("c")} );
    langs.insert(String::from("cfscript"), Language{ key: String::from("cfscript"), label: String::from("cfscript")} );
    langs.insert(String::from("chaiscript"), Language{ key: String::from("chaiscript"), label: String::from("chaiscript")} );
    langs.insert(String::from("cil"), Language{ key: String::from("cil"), label: String::from("cil")} );
    langs.insert(String::from("clike"), Language{ key: String::from("clike"), label: String::from("clike")} );
    langs.insert(String::from("clojure"), Language{ key: String::from("clojure"), label: String::from("clojure")} );
    langs.insert(String::from("cmake"), Language{ key: String::from("cmake"), label: String::from("cmake")} );
    langs.insert(String::from("cobol"), Language{ key: String::from("cobol"), label: String::from("cobol")} );
    langs.insert(String::from("coffeescript"), Language{ key: String::from("coffeescript"), label: String::from("coffeescript")} );
    langs.insert(String::from("concurnas"), Language{ key: String::from("concurnas"), label: String::from("concurnas")} );
    langs.insert(String::from("coq"), Language{ key: String::from("coq"), label: String::from("coq")} );
    langs.insert(String::from("cpp"), Language{ key: String::from("cpp"), label: String::from("cpp")} );
    langs.insert(String::from("crystal"), Language{ key: String::from("crystal"), label: String::from("crystal")} );
    langs.insert(String::from("csharp"), Language{ key: String::from("csharp"), label: String::from("csharp")} );
    langs.insert(String::from("cshtml"), Language{ key: String::from("cshtml"), label: String::from("cshtml")} );
    langs.insert(String::from("csp"), Language{ key: String::from("csp"), label: String::from("csp")} );
    langs.insert(String::from("cssExtras"), Language{ key: String::from("cssExtras"), label: String::from("cssExtras")} );
    langs.insert(String::from("css"), Language{ key: String::from("css"), label: String::from("css")} );
    langs.insert(String::from("csv"), Language{ key: String::from("csv"), label: String::from("csv")} );
    langs.insert(String::from("cypher"), Language{ key: String::from("cypher"), label: String::from("cypher")} );
    langs.insert(String::from("d"), Language{ key: String::from("d"), label: String::from("d")} );
    langs.insert(String::from("dart"), Language{ key: String::from("dart"), label: String::from("dart")} );
    langs.insert(String::from("dataweave"), Language{ key: String::from("dataweave"), label: String::from("dataweave")} );
    langs.insert(String::from("dax"), Language{ key: String::from("dax"), label: String::from("dax")} );
    langs.insert(String::from("dhall"), Language{ key: String::from("dhall"), label: String::from("dhall")} );
    langs.insert(String::from("diff"), Language{ key: String::from("diff"), label: String::from("diff")} );
    langs.insert(String::from("django"), Language{ key: String::from("django"), label: String::from("django")} );
    langs.insert(String::from("dnsZoneFile"), Language{ key: String::from("dnsZoneFile"), label: String::from("dnsZoneFile")} );
    langs.insert(String::from("docker"), Language{ key: String::from("docker"), label: String::from("docker")} );
    langs.insert(String::from("dot"), Language{ key: String::from("dot"), label: String::from("dot")} );
    langs.insert(String::from("ebnf"), Language{ key: String::from("ebnf"), label: String::from("ebnf")} );
    langs.insert(String::from("editorconfig"), Language{ key: String::from("editorconfig"), label: String::from("editorconfig")} );
    langs.insert(String::from("eiffel"), Language{ key: String::from("eiffel"), label: String::from("eiffel")} );
    langs.insert(String::from("ejs"), Language{ key: String::from("ejs"), label: String::from("ejs")} );
    langs.insert(String::from("elixir"), Language{ key: String::from("elixir"), label: String::from("elixir")} );
    langs.insert(String::from("elm"), Language{ key: String::from("elm"), label: String::from("elm")} );
    langs.insert(String::from("erb"), Language{ key: String::from("erb"), label: String::from("erb")} );
    langs.insert(String::from("erlang"), Language{ key: String::from("erlang"), label: String::from("erlang")} );
    langs.insert(String::from("etlua"), Language{ key: String::from("etlua"), label: String::from("etlua")} );
    langs.insert(String::from("excelFormula"), Language{ key: String::from("excelFormula"), label: String::from("excelFormula")} );
    langs.insert(String::from("factor"), Language{ key: String::from("factor"), label: String::from("factor")} );
    langs.insert(String::from("falselang"), Language{ key: String::from("falselang"), label: String::from("falselang")} );
    langs.insert(String::from("firestoreSecurityRules"), Language{ key: String::from("firestoreSecurityRules"), label: String::from("firestoreSecurityRules")} );
    langs.insert(String::from("flow"), Language{ key: String::from("flow"), label: String::from("flow")} );
    langs.insert(String::from("fortran"), Language{ key: String::from("fortran"), label: String::from("fortran")} );
    langs.insert(String::from("fsharp"), Language{ key: String::from("fsharp"), label: String::from("fsharp")} );
    langs.insert(String::from("ftl"), Language{ key: String::from("ftl"), label: String::from("ftl")} );
    langs.insert(String::from("gap"), Language{ key: String::from("gap"), label: String::from("gap")} );
    langs.insert(String::from("gcode"), Language{ key: String::from("gcode"), label: String::from("gcode")} );
    langs.insert(String::from("gdscript"), Language{ key: String::from("gdscript"), label: String::from("gdscript")} );
    langs.insert(String::from("gedcom"), Language{ key: String::from("gedcom"), label: String::from("gedcom")} );
    langs.insert(String::from("gherkin"), Language{ key: String::from("gherkin"), label: String::from("gherkin")} );
    langs.insert(String::from("git"), Language{ key: String::from("git"), label: String::from("git")} );
    langs.insert(String::from("glsl"), Language{ key: String::from("glsl"), label: String::from("glsl")} );
    langs.insert(String::from("gml"), Language{ key: String::from("gml"), label: String::from("gml")} );
    langs.insert(String::from("gn"), Language{ key: String::from("gn"), label: String::from("gn")} );
    langs.insert(String::from("goModule"), Language{ key: String::from("goModule"), label: String::from("goModule")} );
    langs.insert(String::from("go"), Language{ key: String::from("go"), label: String::from("go")} );
    langs.insert(String::from("graphql"), Language{ key: String::from("graphql"), label: String::from("graphql")} );
    langs.insert(String::from("groovy"), Language{ key: String::from("groovy"), label: String::from("groovy")} );
    langs.insert(String::from("haml"), Language{ key: String::from("haml"), label: String::from("haml")} );
    langs.insert(String::from("handlebars"), Language{ key: String::from("handlebars"), label: String::from("handlebars")} );
    langs.insert(String::from("haskell"), Language{ key: String::from("haskell"), label: String::from("haskell")} );
    langs.insert(String::from("haxe"), Language{ key: String::from("haxe"), label: String::from("haxe")} );
    langs.insert(String::from("hcl"), Language{ key: String::from("hcl"), label: String::from("hcl")} );
    langs.insert(String::from("hlsl"), Language{ key: String::from("hlsl"), label: String::from("hlsl")} );
    langs.insert(String::from("hoon"), Language{ key: String::from("hoon"), label: String::from("hoon")} );
    langs.insert(String::from("hpkp"), Language{ key: String::from("hpkp"), label: String::from("hpkp")} );
    langs.insert(String::from("hsts"), Language{ key: String::from("hsts"), label: String::from("hsts")} );
    langs.insert(String::from("http"), Language{ key: String::from("http"), label: String::from("http")} );
    langs.insert(String::from("ichigojam"), Language{ key: String::from("ichigojam"), label: String::from("ichigojam")} );
    langs.insert(String::from("icon"), Language{ key: String::from("icon"), label: String::from("icon")} );
    langs.insert(String::from("icuMessageFormat"), Language{ key: String::from("icuMessageFormat"), label: String::from("icuMessageFormat")} );
    langs.insert(String::from("idris"), Language{ key: String::from("idris"), label: String::from("idris")} );
    langs.insert(String::from("iecst"), Language{ key: String::from("iecst"), label: String::from("iecst")} );
    langs.insert(String::from("ignore"), Language{ key: String::from("ignore"), label: String::from("ignore")} );
    langs.insert(String::from("inform7"), Language{ key: String::from("inform7"), label: String::from("inform7")} );
    langs.insert(String::from("ini"), Language{ key: String::from("ini"), label: String::from("ini")} );
    langs.insert(String::from("io"), Language{ key: String::from("io"), label: String::from("io")} );
    langs.insert(String::from("j"), Language{ key: String::from("j"), label: String::from("j")} );
    langs.insert(String::from("java"), Language{ key: String::from("java"), label: String::from("java")} );
    langs.insert(String::from("javadoc"), Language{ key: String::from("javadoc"), label: String::from("javadoc")} );
    langs.insert(String::from("javadoclike"), Language{ key: String::from("javadoclike"), label: String::from("javadoclike")} );
    langs.insert(String::from("javascript"), Language{ key: String::from("javascript"), label: String::from("javascript")} );
    langs.insert(String::from("javastacktrace"), Language{ key: String::from("javastacktrace"), label: String::from("javastacktrace")} );
    langs.insert(String::from("jexl"), Language{ key: String::from("jexl"), label: String::from("jexl")} );
    langs.insert(String::from("jolie"), Language{ key: String::from("jolie"), label: String::from("jolie")} );
    langs.insert(String::from("jq"), Language{ key: String::from("jq"), label: String::from("jq")} );
    langs.insert(String::from("jsExtras"), Language{ key: String::from("jsExtras"), label: String::from("jsExtras")} );
    langs.insert(String::from("jsTemplates"), Language{ key: String::from("jsTemplates"), label: String::from("jsTemplates")} );
    langs.insert(String::from("jsdoc"), Language{ key: String::from("jsdoc"), label: String::from("jsdoc")} );
    langs.insert(String::from("json"), Language{ key: String::from("json"), label: String::from("json")} );
    langs.insert(String::from("json5"), Language{ key: String::from("json5"), label: String::from("json5")} );
    langs.insert(String::from("jsonp"), Language{ key: String::from("jsonp"), label: String::from("jsonp")} );
    langs.insert(String::from("jsstacktrace"), Language{ key: String::from("jsstacktrace"), label: String::from("jsstacktrace")} );
    langs.insert(String::from("jsx"), Language{ key: String::from("jsx"), label: String::from("jsx")} );
    langs.insert(String::from("julia"), Language{ key: String::from("julia"), label: String::from("julia")} );
    langs.insert(String::from("keepalived"), Language{ key: String::from("keepalived"), label: String::from("keepalived")} );
    langs.insert(String::from("keyman"), Language{ key: String::from("keyman"), label: String::from("keyman")} );
    langs.insert(String::from("kotlin"), Language{ key: String::from("kotlin"), label: String::from("kotlin")} );
    langs.insert(String::from("kumir"), Language{ key: String::from("kumir"), label: String::from("kumir")} );
    langs.insert(String::from("kusto"), Language{ key: String::from("kusto"), label: String::from("kusto")} );
    langs.insert(String::from("latex"), Language{ key: String::from("latex"), label: String::from("latex")} );
    langs.insert(String::from("latte"), Language{ key: String::from("latte"), label: String::from("latte")} );
    langs.insert(String::from("less"), Language{ key: String::from("less"), label: String::from("less")} );
    langs.insert(String::from("lilypond"), Language{ key: String::from("lilypond"), label: String::from("lilypond")} );
    langs.insert(String::from("liquid"), Language{ key: String::from("liquid"), label: String::from("liquid")} );
    langs.insert(String::from("lisp"), Language{ key: String::from("lisp"), label: String::from("lisp")} );
    langs.insert(String::from("livescript"), Language{ key: String::from("livescript"), label: String::from("livescript")} );
    langs.insert(String::from("llvm"), Language{ key: String::from("llvm"), label: String::from("llvm")} );
    langs.insert(String::from("log"), Language{ key: String::from("log"), label: String::from("log")} );
    langs.insert(String::from("lolcode"), Language{ key: String::from("lolcode"), label: String::from("lolcode")} );
    langs.insert(String::from("lua"), Language{ key: String::from("lua"), label: String::from("lua")} );
    langs.insert(String::from("magma"), Language{ key: String::from("magma"), label: String::from("magma")} );
    langs.insert(String::from("makefile"), Language{ key: String::from("makefile"), label: String::from("makefile")} );
    langs.insert(String::from("markdown"), Language{ key: String::from("markdown"), label: String::from("markdown")} );
    langs.insert(String::from("markupTemplating"), Language{ key: String::from("markupTemplating"), label: String::from("markupTemplating")} );
    langs.insert(String::from("markup"), Language{ key: String::from("markup"), label: String::from("markup")} );
    langs.insert(String::from("matlab"), Language{ key: String::from("matlab"), label: String::from("matlab")} );
    langs.insert(String::from("maxscript"), Language{ key: String::from("maxscript"), label: String::from("maxscript")} );
    langs.insert(String::from("mel"), Language{ key: String::from("mel"), label: String::from("mel")} );
    langs.insert(String::from("mermaid"), Language{ key: String::from("mermaid"), label: String::from("mermaid")} );
    langs.insert(String::from("mizar"), Language{ key: String::from("mizar"), label: String::from("mizar")} );
    langs.insert(String::from("mongodb"), Language{ key: String::from("mongodb"), label: String::from("mongodb")} );
    langs.insert(String::from("monkey"), Language{ key: String::from("monkey"), label: String::from("monkey")} );
    langs.insert(String::from("moonscript"), Language{ key: String::from("moonscript"), label: String::from("moonscript")} );
    langs.insert(String::from("n1ql"), Language{ key: String::from("n1ql"), label: String::from("n1ql")} );
    langs.insert(String::from("n4js"), Language{ key: String::from("n4js"), label: String::from("n4js")} );
    langs.insert(String::from("nand2tetrisHdl"), Language{ key: String::from("nand2tetrisHdl"), label: String::from("nand2tetrisHdl")} );
    langs.insert(String::from("naniscript"), Language{ key: String::from("naniscript"), label: String::from("naniscript")} );
    langs.insert(String::from("nasm"), Language{ key: String::from("nasm"), label: String::from("nasm")} );
    langs.insert(String::from("neon"), Language{ key: String::from("neon"), label: String::from("neon")} );
    langs.insert(String::from("nevod"), Language{ key: String::from("nevod"), label: String::from("nevod")} );
    langs.insert(String::from("nginx"), Language{ key: String::from("nginx"), label: String::from("nginx")} );
    langs.insert(String::from("nim"), Language{ key: String::from("nim"), label: String::from("nim")} );
    langs.insert(String::from("nix"), Language{ key: String::from("nix"), label: String::from("nix")} );
    langs.insert(String::from("nsis"), Language{ key: String::from("nsis"), label: String::from("nsis")} );
    langs.insert(String::from("objectivec"), Language{ key: String::from("objectivec"), label: String::from("objectivec")} );
    langs.insert(String::from("ocaml"), Language{ key: String::from("ocaml"), label: String::from("ocaml")} );
    langs.insert(String::from("opencl"), Language{ key: String::from("opencl"), label: String::from("opencl")} );
    langs.insert(String::from("openqasm"), Language{ key: String::from("openqasm"), label: String::from("openqasm")} );
    langs.insert(String::from("oz"), Language{ key: String::from("oz"), label: String::from("oz")} );
    langs.insert(String::from("parigp"), Language{ key: String::from("parigp"), label: String::from("parigp")} );
    langs.insert(String::from("parser"), Language{ key: String::from("parser"), label: String::from("parser")} );
    langs.insert(String::from("pascal"), Language{ key: String::from("pascal"), label: String::from("pascal")} );
    langs.insert(String::from("pascaligo"), Language{ key: String::from("pascaligo"), label: String::from("pascaligo")} );
    langs.insert(String::from("pcaxis"), Language{ key: String::from("pcaxis"), label: String::from("pcaxis")} );
    langs.insert(String::from("peoplecode"), Language{ key: String::from("peoplecode"), label: String::from("peoplecode")} );
    langs.insert(String::from("perl"), Language{ key: String::from("perl"), label: String::from("perl")} );
    langs.insert(String::from("phpExtras"), Language{ key: String::from("phpExtras"), label: String::from("phpExtras")} );
    langs.insert(String::from("php"), Language{ key: String::from("php"), label: String::from("php")} );
    langs.insert(String::from("phpdoc"), Language{ key: String::from("phpdoc"), label: String::from("phpdoc")} );
    langs.insert(String::from("plsql"), Language{ key: String::from("plsql"), label: String::from("plsql")} );
    langs.insert(String::from("powerquery"), Language{ key: String::from("powerquery"), label: String::from("powerquery")} );
    langs.insert(String::from("powershell"), Language{ key: String::from("powershell"), label: String::from("powershell")} );
    langs.insert(String::from("processing"), Language{ key: String::from("processing"), label: String::from("processing")} );
    langs.insert(String::from("prolog"), Language{ key: String::from("prolog"), label: String::from("prolog")} );
    langs.insert(String::from("promql"), Language{ key: String::from("promql"), label: String::from("promql")} );
    langs.insert(String::from("properties"), Language{ key: String::from("properties"), label: String::from("properties")} );
    langs.insert(String::from("protobuf"), Language{ key: String::from("protobuf"), label: String::from("protobuf")} );
    langs.insert(String::from("psl"), Language{ key: String::from("psl"), label: String::from("psl")} );
    langs.insert(String::from("pug"), Language{ key: String::from("pug"), label: String::from("pug")} );
    langs.insert(String::from("puppet"), Language{ key: String::from("puppet"), label: String::from("puppet")} );
    langs.insert(String::from("pure"), Language{ key: String::from("pure"), label: String::from("pure")} );
    langs.insert(String::from("purebasic"), Language{ key: String::from("purebasic"), label: String::from("purebasic")} );
    langs.insert(String::from("purescript"), Language{ key: String::from("purescript"), label: String::from("purescript")} );
    langs.insert(String::from("python"), Language{ key: String::from("python"), label: String::from("python")} );
    langs.insert(String::from("q"), Language{ key: String::from("q"), label: String::from("q")} );
    langs.insert(String::from("qml"), Language{ key: String::from("qml"), label: String::from("qml")} );
    langs.insert(String::from("qore"), Language{ key: String::from("qore"), label: String::from("qore")} );
    langs.insert(String::from("qsharp"), Language{ key: String::from("qsharp"), label: String::from("qsharp")} );
    langs.insert(String::from("r"), Language{ key: String::from("r"), label: String::from("r")} );
    langs.insert(String::from("racket"), Language{ key: String::from("racket"), label: String::from("racket")} );
    langs.insert(String::from("reason"), Language{ key: String::from("reason"), label: String::from("reason")} );
    langs.insert(String::from("regex"), Language{ key: String::from("regex"), label: String::from("regex")} );
    langs.insert(String::from("rego"), Language{ key: String::from("rego"), label: String::from("rego")} );
    langs.insert(String::from("renpy"), Language{ key: String::from("renpy"), label: String::from("renpy")} );
    langs.insert(String::from("rest"), Language{ key: String::from("rest"), label: String::from("rest")} );
    langs.insert(String::from("rip"), Language{ key: String::from("rip"), label: String::from("rip")} );
    langs.insert(String::from("roboconf"), Language{ key: String::from("roboconf"), label: String::from("roboconf")} );
    langs.insert(String::from("robotframework"), Language{ key: String::from("robotframework"), label: String::from("robotframework")} );
    langs.insert(String::from("ruby"), Language{ key: String::from("ruby"), label: String::from("ruby")} );
    langs.insert(String::from("rust"), Language{ key: String::from("rust"), label: String::from("rust")} );
    langs.insert(String::from("sas"), Language{ key: String::from("sas"), label: String::from("sas")} );
    langs.insert(String::from("sass"), Language{ key: String::from("sass"), label: String::from("sass")} );
    langs.insert(String::from("scala"), Language{ key: String::from("scala"), label: String::from("scala")} );
    langs.insert(String::from("scheme"), Language{ key: String::from("scheme"), label: String::from("scheme")} );
    langs.insert(String::from("scss"), Language{ key: String::from("scss"), label: String::from("scss")} );
    langs.insert(String::from("shellSession"), Language{ key: String::from("shellSession"), label: String::from("shellSession")} );
    langs.insert(String::from("smali"), Language{ key: String::from("smali"), label: String::from("smali")} );
    langs.insert(String::from("smalltalk"), Language{ key: String::from("smalltalk"), label: String::from("smalltalk")} );
    langs.insert(String::from("smarty"), Language{ key: String::from("smarty"), label: String::from("smarty")} );
    langs.insert(String::from("sml"), Language{ key: String::from("sml"), label: String::from("sml")} );
    langs.insert(String::from("solidity"), Language{ key: String::from("solidity"), label: String::from("solidity")} );
    langs.insert(String::from("solutionFile"), Language{ key: String::from("solutionFile"), label: String::from("solutionFile")} );
    langs.insert(String::from("soy"), Language{ key: String::from("soy"), label: String::from("soy")} );
    langs.insert(String::from("sparql"), Language{ key: String::from("sparql"), label: String::from("sparql")} );
    langs.insert(String::from("splunkSpl"), Language{ key: String::from("splunkSpl"), label: String::from("splunkSpl")} );
    langs.insert(String::from("sqf"), Language{ key: String::from("sqf"), label: String::from("sqf")} );
    langs.insert(String::from("sql"), Language{ key: String::from("sql"), label: String::from("sql")} );
    langs.insert(String::from("squirrel"), Language{ key: String::from("squirrel"), label: String::from("squirrel")} );
    langs.insert(String::from("stan"), Language{ key: String::from("stan"), label: String::from("stan")} );
    langs.insert(String::from("stylus"), Language{ key: String::from("stylus"), label: String::from("stylus")} );
    langs.insert(String::from("swift"), Language{ key: String::from("swift"), label: String::from("swift")} );
    langs.insert(String::from("systemd"), Language{ key: String::from("systemd"), label: String::from("systemd")} );
    langs.insert(String::from("t4Cs"), Language{ key: String::from("t4Cs"), label: String::from("t4Cs")} );
    langs.insert(String::from("t4Templating"), Language{ key: String::from("t4Templating"), label: String::from("t4Templating")} );
    langs.insert(String::from("t4Vb"), Language{ key: String::from("t4Vb"), label: String::from("t4Vb")} );
    langs.insert(String::from("tap"), Language{ key: String::from("tap"), label: String::from("tap")} );
    langs.insert(String::from("tcl"), Language{ key: String::from("tcl"), label: String::from("tcl")} );
    langs.insert(String::from("textile"), Language{ key: String::from("textile"), label: String::from("textile")} );
    langs.insert(String::from("toml"), Language{ key: String::from("toml"), label: String::from("toml")} );
    langs.insert(String::from("tremor"), Language{ key: String::from("tremor"), label: String::from("tremor")} );
    langs.insert(String::from("tsx"), Language{ key: String::from("tsx"), label: String::from("tsx")} );
    langs.insert(String::from("tt2"), Language{ key: String::from("tt2"), label: String::from("tt2")} );
    langs.insert(String::from("turtle"), Language{ key: String::from("turtle"), label: String::from("turtle")} );
    langs.insert(String::from("twig"), Language{ key: String::from("twig"), label: String::from("twig")} );
    langs.insert(String::from("typescript"), Language{ key: String::from("typescript"), label: String::from("typescript")} );
    langs.insert(String::from("typoscript"), Language{ key: String::from("typoscript"), label: String::from("typoscript")} );
    langs.insert(String::from("unrealscript"), Language{ key: String::from("unrealscript"), label: String::from("unrealscript")} );
    langs.insert(String::from("uorazor"), Language{ key: String::from("uorazor"), label: String::from("uorazor")} );
    langs.insert(String::from("uri"), Language{ key: String::from("uri"), label: String::from("uri")} );
    langs.insert(String::from("v"), Language{ key: String::from("v"), label: String::from("v")} );
    langs.insert(String::from("vala"), Language{ key: String::from("vala"), label: String::from("vala")} );
    langs.insert(String::from("vbnet"), Language{ key: String::from("vbnet"), label: String::from("vbnet")} );
    langs.insert(String::from("velocity"), Language{ key: String::from("velocity"), label: String::from("velocity")} );
    langs.insert(String::from("verilog"), Language{ key: String::from("verilog"), label: String::from("verilog")} );
    langs.insert(String::from("vhdl"), Language{ key: String::from("vhdl"), label: String::from("vhdl")} );
    langs.insert(String::from("vim"), Language{ key: String::from("vim"), label: String::from("vim")} );
    langs.insert(String::from("visualBasic"), Language{ key: String::from("visualBasic"), label: String::from("visualBasic")} );
    langs.insert(String::from("warpscript"), Language{ key: String::from("warpscript"), label: String::from("warpscript")} );
    langs.insert(String::from("wasm"), Language{ key: String::from("wasm"), label: String::from("wasm")} );
    langs.insert(String::from("webIdl"), Language{ key: String::from("webIdl"), label: String::from("webIdl")} );
    langs.insert(String::from("wiki"), Language{ key: String::from("wiki"), label: String::from("wiki")} );
    langs.insert(String::from("wolfram"), Language{ key: String::from("wolfram"), label: String::from("wolfram")} );
    langs.insert(String::from("wren"), Language{ key: String::from("wren"), label: String::from("wren")} );
    langs.insert(String::from("xeora"), Language{ key: String::from("xeora"), label: String::from("xeora")} );
    langs.insert(String::from("xmlDoc"), Language{ key: String::from("xmlDoc"), label: String::from("xmlDoc")} );
    langs.insert(String::from("xojo"), Language{ key: String::from("xojo"), label: String::from("xojo")} );
    langs.insert(String::from("xquery"), Language{ key: String::from("xquery"), label: String::from("xquery")} );
    langs.insert(String::from("yaml"), Language{ key: String::from("yaml"), label: String::from("yaml")} );
    langs.insert(String::from("yang"), Language{ key: String::from("yang"), label: String::from("yang")} );
    langs.insert(String::from("zig"), Language{ key: String::from("zig"), label: String::from("zig")} );

    langs
}
