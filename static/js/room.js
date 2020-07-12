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
	search(query, callback =>
	{
		resetSearchResults()
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
		console.log("Warning: One or more values missing, not adding result")
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
	thumbnail.src = thumbnailSrc
	searchInfo.appendChild(thumbnail)

	// Search info: Title
	const title = document.createElement("h5")
	if (titleText.length > 60) {
		titleText = titleText.subString(0, 60) + "..."
	}
	title.textContent = titleText
	searchInfo.appendChild(title)

	// Search info: Description
	const description = document.createElement("p")
	description.textContent = descriptionText
	searchInfo.appendChild(description)

	// Add queue container
	const queue = document.createElement("div")
	queue.className = "addQueueContainer"
	queue.onclick = () => addVideo(videoId, titleText)
	searchResult.appendChild(queue)

	// Add queue container: Img
	const queueImg = document.createElement("img")
	queueImg.src = "/img/icon/addQueue.png"
	queue.appendChild(queueImg)

	// Add queue container: Span
	const queueTxt = document.createElement("span")
	queueTxt.textContent = "Add to Queue"
	queue.appendChild(queueTxt)

	// Add result to results
	searchResults.appendChild(searchResult)
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
	addComment(data.avatar_url ? data.avatar_url : data.type, data.value)
}

socket.onerror = event => {
	addComment("error", "Connection failed")
}

socket.onclose = () => {
	addComment("status", "Disconnected")
	id("commentEntry").disabled = true
}

function addVideo(id, title) {
	socket.emit("video",
		{
			type: "add",
			id: id,
			title: title
		})
}