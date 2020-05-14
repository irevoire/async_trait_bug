#[async_trait::async_trait]
trait Bar {
    async fn bar();
}

struct Baz {}

impl Baz {
    const FOO: u32 = 3;
}

#[async_trait::async_trait]
impl Bar for Baz {
    async fn bar() {
        println!("bar: {}", Self::FOO);
        // The following code fix the issue
        // let tmp = Self::FOO;
        // println!("bar: {}", tmp);
    }
}

fn main() {
    Baz::bar();
}
