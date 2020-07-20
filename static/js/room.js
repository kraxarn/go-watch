id("videoSearch").addEventListener("input", () =>
{
	// Get search query
	const query = id("videoSearch").value

	// Check is search is too short
	if (query.length < 3) {
		resetSearchResults()
		return
	}

	// Search
	const videoSearch = id("videoSearch")
	videoSearch.disabled = true
	search(query, callback =>
	{
		resetSearchResults()
		videoSearch.disabled = false
		callback.forEach(item =>
		{
			addSearchResult(item.thumbnail, item.title, item.description, item.id)
		})
	})
})

function search(query, callback)
{
	fetch("/api/search", {
		method: "POST",
		headers: {
			"Content-Type": "application/json"
		},
		body: JSON.stringify({
			q: query
		})
	})
		.then(response => response.json())
		.then(json =>
		{
			if (json.error) {
				callback(null)
			} else {
				callback(json)
			}
		})
}

function addSearchResult(thumbnailSrc, titleText, descriptionText, videoId)
{
	if (!thumbnailSrc || !titleText || !descriptionText) {
		return
	}

	// Main search result
	const searchResult = document.createElement("div")
	searchResult.className = "searchResult"

	// Search info
	const searchInfo = document.createElement("div")
	searchInfo.className = "searchInfo"
	searchResult.appendChild(searchInfo)

	// Search info: Thumbnail
	const thumbnail = document.createElement("img")
	thumbnail.className = "searchThumbnail"
	thumbnail.src = `https://i.ytimg.com/vi/${videoId}/mqdefault.jpg`
	searchInfo.appendChild(thumbnail)

	// Search info: Title
	const title = document.createElement("h5")
	if (titleText.length > 60) {
		titleText = titleText.substring(0, 60) + "..."
	}
	title.textContent = titleText
	searchInfo.appendChild(title)

	// Search info: Description
	const description = document.createElement("p")
	description.textContent = descriptionText
	searchInfo.appendChild(description)

	// Add queue container: Img
	const queueImg = document.createElement("img")
	queueImg.className = "queueImg"
	queueImg.src = "/img/1f4e5.svg"
	queueImg.title = "Add to queue"
	queueImg.onclick = () => addVideo(videoId, titleText)
	searchResult.appendChild(queueImg)

	// Add result to results
	id("searchResults").appendChild(searchResult)
}

function resetSearchResults()
{
	const results = id("searchResults")
	while (results.firstChild) {
		results.removeChild(results.firstChild)
	}
}

function toggleVideoSize()
{
	const dropdown = id("videoSizeDropdown")
	const visible = dropdown.style.visibility === "visible"

	dropdown.style.visibility = visible ? "hidden" : "visible"
}

function setVideoSize(width, height)
{
	id("video").style.width  = width + "px"
	id("video").style.height = height + "px"

	id("videoSizeString").textContent = height
}

function addComment(type, message)
{
	const icon = type === "status"
		? "1f4e2" : type === "error"
			? "274c" : type === "message"
				? "1f4ac" : type

	const comment = document.createElement("div")
	comment.className = "comment"

	const img = document.createElement("img")
	img.src = `/img//${icon}.svg`
	comment.appendChild(img)

	const msg = document.createElement("span")
	msg.textContent = message
	comment.appendChild(msg)

	id("commentsContainer").appendChild(comment)
}

function addTestComments()
{
	addComment("status", "User1 joined")
	addComment("status", "User2 joined")
	addComment("message", "User1: How exciting, some sample text")
	addComment("message", "User2: I know right? It even wraps quite nicely when typing long messages.")
	addComment("playback", "User1 started playback")
}

function addQueueItem(thumbnailSrc, titleText, videoId)
{
	// Hide no queue message
	id("noQueueItems").style.display = "none"

	// Main item container
	const item = document.createElement("div")
	item.className = "queueItem"
	item.id = videoId

	// Background thumbnail
	const thumbnail = document.createElement("img")
	thumbnail.className = "queueThumbnail"
	thumbnail.src = thumbnailSrc
	item.appendChild(thumbnail)

	// Video title
	const title = document.createElement("h5")
	// TODO: 60 is probably too long
	if (titleText.length > 60) {
		titleText = titleText.subString(0, 60) + "..."
	}
	title.textContent = titleText
	item.appendChild(title)

	// Remove button
	const rem = document.createElement("img")
	rem.className = "queueRemove"
	rem.src = "/img/icon/remove.png"
	item.appendChild(rem)

	id("queueContainer").appendChild(item)
}

// Testing only, use array later
function getQueuedItems()
{
	const children = id("queueContainer").children
	for (let i = 0; i < children.length; i++) {
		console.log(children[i].id)
	}
}

const entry = id("commentEntry")

const video = document.querySelector("video")
const audio = document.querySelector("audio")

audio.onplay = () =>
	video.play().then(_ =>
		video.currentTime = audio.currentTime)

audio.onpause = () => video.pause()

entry.addEventListener("keypress", event =>
{
	if (event.key === "Enter") {
		if (entry.value.length > 0 && entry.value.length < 256) {
			socket.send(entry.value)
		}
		entry.value = null
	}
})

const socket = new WebSocket(`ws://${location.host}/chat`)

socket.onopen = () => {
	addComment("status", "Connected")
	id("commentEntry").disabled = false
}

socket.onmessage = event => {
	const data = JSON.parse(event.data)
	switch (data.type) {
		case "message":
			addComment(data["avatar_url"], data["message"])
			break

		case "video":
			audio.currentTime = 0
			video.src = data.video
			audio.src = data.audio
			audio.play()
			break
	}
}

socket.onerror = event => {
	addComment("error", "Connection failed")
}

socket.onclose = () => {
	addComment("status", "Disconnected")
	id("commentEntry").disabled = true
}

function addVideo(id, title) {
	socket.send(`/video ${id}`)
}