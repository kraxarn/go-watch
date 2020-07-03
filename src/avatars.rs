use std::collections::HashMap;

pub const AVATAR_VALUES: [(u32, &'static str); 29] = [
	(0x1f43b, "bear"),
	(0x1f417, "boar"),
	(0x1f431, "cat"),
	(0x1f414, "chicken"),
	(0x1f42e, "cow"),
	(0x1f98c, "deer"),
	(0x1f436, "dog"),
	(0x1f432, "dragon"),
	(0x1f985, "eagle"),
	(0x1f98a, "fox"),
	(0x1f438, "frog"),
	(0x1f992, "giraffe"),
	(0x1f98d, "gorilla"),
	(0x1f439, "hamster"),
	(0x1f434, "horse"),
	(0x1f428, "koala"),
	(0x1f981, "lion"),
	(0x1f435, "monkey"),
	(0x1f42d, "mouse"),
	(0x1f43c, "panda"),
	(0x1f437, "pig"),
	(0x1f4a9, "poop"),
	(0x1f430, "rabbit"),
	(0x1f99d, "raccoon"),
	(0x1f98f, "rhinoceros"),
	(0x1f42f, "tiger"),
	(0x1f984, "unicorn"),
	(0x1f43a, "wolf"),
	(0x1f993, "zebra"),
];

lazy_static! {
	pub static ref AVATAR_MAP: HashMap<u32, &'static str> = AVATAR_VALUES.iter().cloned().collect();
}