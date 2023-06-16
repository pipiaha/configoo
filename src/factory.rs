// enum 工厂模式

use std::rc::Rc;

trait MyTrait {
    fn do_something(&self);
}

struct StructA;

impl MyTrait for StructA {
    fn do_something(&self) {
        println!("Calling do_something() on StructA");
    }
}

struct StructB;

impl MyTrait for StructB {
    fn do_something(&self) {
        println!("Calling do_something() on StructB");
    }
}

enum MyEnum {
    VariantA(Rc<dyn MyTrait>),
    VariantB(Rc<dyn MyTrait>),
}

impl MyTrait for MyEnum {
    fn do_something(&self) {
        match self {
            MyEnum::VariantA(struct_a) => struct_a.do_something(),
            MyEnum::VariantB(struct_b) => struct_b.do_something(),
        }
    }
}

enum MyEnumFactory {
    VariantA,
    VariantB,
}

impl MyEnumFactory {
    fn create(&self) -> MyEnum {
        match self {
            MyEnumFactory::VariantA => MyEnum::VariantA(Rc::new(StructA)),
            MyEnumFactory::VariantB => MyEnum::VariantB(Rc::new(StructB)),
        }
    }
}

fn main() {
    let my_enum_factory = MyEnumFactory::VariantA;
    let my_enum = my_enum_factory.create();
    my_enum.do_something();

    let my_enum_factory = MyEnumFactory::VariantB;
    let my_enum = my_enum_factory.create();
    my_enum.do_something();
}
