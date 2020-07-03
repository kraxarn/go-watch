const id = (elementId) => document.getElementById(elementId)

// Open avatar selection
id("avatar").onclick = () => {
    const avatarSelect = id("avatarSelect")
	if (avatarSelect.style.display === "block") {
		avatarSelect.style.display = "none"
	} else {
		avatarSelect.style.display = "block"
	}
}

// Switching between viewing/editing username
id("username").onclick = () => {
	id("username").style.display = "none"
	id("nameEntry").style.display = "flex"
}

id("saveName").onclick = () => {
    const username = id("username")
	username.style.display = "block"
	id("nameEntry").style.display = "none"
	username.textContent = nameInput.value
	setName()
}

// Show room name selection
id("createRoom").onclick = () => {
	id("createRoom").style.display = "none"
	id("roomEntry").style.display = "flex"
}

// Save room button
id("saveRoom").onclick = () => {
	const name = id("roomInput").value.replace(/\s/g, "").toLowerCase()
	if (name.length < 3 || name.length > 16) {
		return
	}
	console.log("Room name: %s", name)
}

// Update avatar image
const setAvatar = (name) => {
	id("avatar").src = `img/avatar/${name}.png`
	id("avatarSelect").style.display = "none"

	updateUserInfo(`avatar=${name}`)
}

// Update name
const setName = () => {
    const nameInput = id("nameInput")
	if (nameInput.value.length < 3) {
		return
	}
	updateUserInfo(`name=${nameInput.value}`)
}

const updateUserInfo = (values) =>
	fetch(`/api/setUserInfo?${values}`)
		.then(response => response.json())
		.then(json => {
			if (json.error) {
				console.log(json.message)
			} else {
				console.log("Update successful")
			}
		})