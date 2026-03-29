use wit_bindgen::generate;

generate!({
    inline: r"
        package my:test;

        world my-world {
            export add: func(a: f32, b: f32) -> f32;
        }
    ",
});

struct MyComponent;

impl Guest for MyComponent {
    fn add(a: f32, b: f32) -> f32 {a + b}
}

export!(MyComponent);