// Open avatar selection
getById("avatar").onclick = () => {
    const avatarSelect = getById("avatarSelect")
	if (avatarSelect.style.display === "block") {
		avatarSelect.style.display = "none"
	} else {
		avatarSelect.style.display = "block"
	}
}

// Switching between viewing/editing username
getById("username").onclick = () => {
	getById("username").style.display = "none"
	getById("nameEntry").style.display = "flex"
}

getById("saveName").onclick = () => {
    const username = getById("username")
	username.style.display = "block"
	getById("nameEntry").style.display = "none"
	username.textContent = getById("nameInput").value
	setName()
}

// Show room name selection
getById("createRoom").onclick = () => {
	getById("createRoom").style.display = "none"
	getById("roomEntry").style.display = "flex"
}

// Save room button
getById("saveRoom").onclick = () => {
	const name = getById("roomInput").value.replace(/\s/g, "-").toLowerCase()
	if (name.length < 3 || name.length > 16) {
		return
	}

	get(`/chat/exists/${name}`)
		.then(json => {
			if (json["exists"]) {
				showError("a room with that name already exists")
			} else {
				showError()
				location.href = `/watch/room/${name}`
			}
		})
}

// Update avatar image
const setAvatar = name => {
	getById("avatar").src = `img/${parseInt(name).toString(16)}.svg`
	getById("avatarSelect").style.display = "none"

	updateUserInfo({
		avatar: name
	})
}

// Update name
const setName = () => {
    const nameInput = getById("nameInput")
	if (nameInput.value.length < 3) {
		return
	}
	updateUserInfo({
		name: nameInput.value
	})
}

const updateUserInfo = body =>
	fetch("/user/update", {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify(body)
	})
		.then(response => response.json())
		.then(json => {
			if (json.error) {
				showError(json.error)
			} else {
				showError()
				getById("avatar").src = `/img/avatar/${parseInt(json.user.avatar).toString(16)}.svg`
				getById("nameInput").value = json.user.name
			}
		})
		.catch(err => showError(err))

const showError = err => {
	const error = getById("error")
	error.textContent = `error: ${err}`
	error.style.display = err ? "block" : "none"
}