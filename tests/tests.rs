#![feature(macro_vis_matcher)]

#[macro_use]
extern crate smart_default;

#[test]
fn test_unit() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo;

    assert!(Foo::default() == Foo);
}

#[test]
fn test_tuple() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo (
        #[default = "10"]
        i32,
        #[default = "20"]
        i32,
        // No default
        i32,
    );

    assert!(Foo::default() == Foo(10, 20, 0));
}

#[test]
fn test_struct() {
    #[derive(PartialEq, SmartDefault)]
    struct Foo {
        #[default = "10"]
        x: i32,
        #[default = "20"]
        y: i32,
        // No default
        z: i32,
    }

    assert!(Foo::default() == Foo { x: 10, y: 20, z: 0 });
}

#[test]
fn test_enum_of_units() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar,
        #[default]
        Baz,
        #[allow(dead_code)]
        Qux,
    }

    assert!(Foo::default() == Foo::Baz);
}

#[test]
fn test_enum_of_tuples() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar(i32),
        #[default]
        Baz(#[default = "10"] i32, i32),
        #[allow(dead_code)]
        Qux(i32),
    }

    assert!(Foo::default() == Foo::Baz(10, 0));
}

#[test]
fn test_enum_of_structs() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar {
            x: i32,
        },
        #[default]
        Baz {
            #[default = "10"]
            y: i32,
            z: i32,
        },
        #[allow(dead_code)]
        Qux {
            w: i32,
        },
    }

    assert!(Foo::default() == Foo::Baz { y: 10, z: 0 });
}

#[test]
fn test_enum_mixed() {
    #[derive(PartialEq, SmartDefault)]
    pub enum Foo {
        #[allow(dead_code)]
        Bar,
        #[default]
        Baz(#[default = "10"] i32),
        #[allow(dead_code)]
        Qux {
            w: i32,
        },
    }

    assert!(Foo::default() == Foo::Baz(10));
}