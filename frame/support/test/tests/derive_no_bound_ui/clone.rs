trait Config {
	type C;
}

#[derive(frame_support::CloneNoBound)]
struct Foo<T: Config> {
	c: T::C,
}

fn main() {}
