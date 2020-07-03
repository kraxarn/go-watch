/*
fn avatar_images<'a>() -> [&'a str; 29] {
	[
		"1f43b", "1f417", "1f431",
		"1f414", "1f42e", "1f98c",
		"1f436", "1f432", "1f985",
		"1f98a", "1f438", "1f992",
		"1f98d", "1f439", "1f434",
		"1f428", "1f981", "1f435",
		"1f42d", "1f43c", "1f437",
		"1f4a9", "1f430", "1f99d",
		"1f98f", "1f42f", "1f984",
		"1f43a", "1f993"
	]
}
 */

pub fn avatars<'a>() -> Vec<(&'a str, &'a str)> {
	vec![
		("1f43b", "bear"),
		("1f417", "boar"),
		("1f431", "cat"),
		("1f414", "chicken"),
		("1f42e", "cow"),
		("1f98c", "deer"),
		("1f436", "dog"),
		("1f432", "dragon"),
		("1f985", "eagle"),
		("1f98a", "fox"),
		("1f438", "frog"),
		("1f992", "giraffe"),
		("1f98d", "gorilla"),
		("1f439", "hamster"),
		("1f434", "horse"),
		("1f428", "koala"),
		("1f981", "lion"),
		("1f435", "monkey"),
		("1f42d", "mouse"),
		("1f43c", "panda"),
		("1f437", "pig"),
		("1f4a9", "poop"),
		("1f430", "rabbit"),
		("1f99d", "raccoon"),
		("1f98f", "rhinoceros"),
		("1f42f", "tiger"),
		("1f984", "unicorn"),
		("1f43a", "wolf"),
		("1f993", "zebra"),
	]
}