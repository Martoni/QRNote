
slint::slint!{
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}


fn main() {
    println!("Hello, world!");
    HelloWorld::new().unwrap().run().unwrap();
}
