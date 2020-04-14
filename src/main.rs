
extern crate clang;


use clang::{Clang, Index, Parser, TranslationUnit, EntityKind, Entity, Accessibility};
use std::path::{PathBuf, Path};
use std::{fs};
use std::env;

fn process_tree(entity:&Entity, file_name: &str) {
    match entity.get_kind() {
        EntityKind::ClassDecl => { 
            process_class(entity, file_name);
        } ,
        EntityKind::StructDecl => {
            process_struct(entity, file_name);
        },
        EntityKind::Namespace => {
            for c in entity.get_children() {
                process_tree(&c, file_name);
            }
        },
        _ => ()
    }
}

fn process_class(entity:&Entity, file_name: &str) {
    let class_name = entity.get_display_name().unwrap_or("[nonname]".to_owned());
    
    if class_name.starts_with("__") {
        return;
    }

    let full_name = entity.get_type().unwrap().get_display_name();

    //println!("class name : {:?}", entity.get_canonical_entity()());
    // println!("class name : {:?}", name_too);

    for e in entity.get_children() {
        match e.get_kind() {
            EntityKind::Method => {
                let access = e.get_accessibility().unwrap();
                if access ==  Accessibility::Public {
                    let is_static = if e.is_static_method() {"::"} else {"."};
                    println!("{}#{}{}{}",file_name, full_name, is_static, e.get_display_name().unwrap().to_string());
                }
            }, 
            EntityKind::ClassDecl => {
                process_tree(&e, file_name);
            },
            _ => {
                // println!("!!!{:?} {:?}", e.get_kind(), e.get_name().unwrap_or("[noname]".to_owned()));
            }
        }
    }
}

fn process_struct(entity: &Entity, file_name: &str ){

    for  c in entity.get_children() {
        match c.get_kind() {
            EntityKind::FieldDecl => {
                c.get_display_name();
            },
            _ => {}
        }
    }
}

fn process_file<P: AsRef<Path>, A: AsRef<str> >(path : P, index : &Index, args:&Vec<A> ) {

    println!("--- Processing path `{}`", path.as_ref().display());

    let mut parser :Parser = index.parser(path.as_ref());
    parser.arguments(args);

    let parse_result = parser.parse();

    if parse_result.is_err() {
        let err = parse_result.err().unwrap();
        println!("process_file: Error: {}",err);
        return
    }

    let tu = parse_result.unwrap();
    let children = tu.get_entity().get_children();

    let file_name = path.as_ref().file_name().unwrap();

    for x in children {
        let kind = x.get_kind();

        match kind {
            EntityKind::FunctionDecl => {
                let name = x.get_display_name().unwrap_or("[noname]".to_owned());
                if !name.starts_with("__") && !name.starts_with("_mm_"){
                    println!("{}:{:?}", file_name.to_str().unwrap(), name);
                }
            },
            EntityKind::ClassDecl => {
                process_tree(&x, file_name.to_str().unwrap());
            },
            EntityKind::Namespace => {
                process_tree(&x, file_name.to_str().unwrap());
            },
            _ => {
                //println!("!!!skip {:?} / {:?}", kind, x);
            }
        }
    }

}

fn process_path<A: AsRef<str> > (dst : &Path, index : &Index, args: &Vec<A> ) {

    if dst.is_dir() {
        for f in fs::read_dir(dst).unwrap() {
            let file_path = f.unwrap().path();
            
            if file_path.is_file() && file_path.to_str().unwrap().ends_with(".h")  {
                process_file(file_path, index, args);
            } else if file_path.is_dir() {
                process_path(&file_path, index, args);
            }
        }
    } else {
        process_file(dst, &index, args);
    }
}

fn main() {

    let mut args = env::args().collect::<Vec<String>>();

    //args.push("-xc++".to_owned());
    args.push("-xc++-header".to_owned());
    args.push("-v".to_owned());
    //args.push("-I/home/jiang/Github/cocos2d-x/cocos".to_owned());
    //args.push("-I/home/jiang/Github/cocos2d-x/external/linux-specific/fmod/include".to_owned());
    //args.push("-I/home/jiang/Github/cocos2d-x/external/glfw3/include/linux".to_owned());
    //args.push("-DLINUX".to_owned());

    args.push("-I/Users/pt/Github/cocos2d-x-lite/cocos".to_owned());
    args.push("-I/Users/pt/Github/cocos2d-x-lite/external/mac/include".to_owned());
    args.push("-I/Users/pt/Github/cocos2d-x-lite/cocos/renderer".to_owned());


    args.push("-I/usr/local/include".to_owned());
    args.push("/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/include".to_owned());
    args.push("/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/11.0.3/include".to_owned());

    args.push("-std=c++14".to_owned());

    println!("args {:?}", args);

    let clang = Clang::new().unwrap();

    let index = Index::new(&clang, false, true);

    //let dst = Path::new("./res/hello.cpp");
    // let dst1 = Path::new("/home/jiang/Github/cocos2d-x/cocos/scripting/lua-bindings/auto");
    // process_path(dst1, &index);
    // let dst2 = Path::new("/home/jiang/Github/cocos2d-x/cocos/scripting/lua-bindings/manual");
    // process_path(dst2, &index);

    //let dst2 = Path::new("/home/jiang/Github/cocos2d-x/cocos/cocos2d.h");
    let dst2 = Path::new("/Users/pt/Github/cocos2d-x-lite/cocos/renderer/core/CoreDef.h");
    process_path(dst2, &index, &args);

}
