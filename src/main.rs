trait Foo {
    const FOO: u32;
}

#[async_trait::async_trait]
trait Bar {
    async fn bar();
}

struct Baz {}

impl Foo for Baz {
    const FOO: u32 = 3;
}

#[async_trait::async_trait]
impl Bar for Baz {
    async fn bar() {
        println!("bar: {}", Self::FOO);
    }
}

impl Baz {
    fn baz() {
        println!("baz: {}", Self::FOO);
    }
}

fn main() {
    Baz::baz();
    Baz::bar();
}
