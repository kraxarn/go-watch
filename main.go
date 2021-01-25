package watch

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"github.com/kraxarn/website/config"
	"github.com/kraxarn/website/user"
	"net/http"
)

func HtmlFiles() []string {
	return []string{
		"watch/html/watch.gohtml",
		"watch/html/room.gohtml",
	}
}

func Route(router *gin.Engine, token *config.Token) {
	// Main state handler
	watch := Watch{
		token: token,
	}

	// Static files
	for _, folder := range []string{
		"css", "img", "js",
	} {
		path := fmt.Sprintf("watch/%s", folder)
		router.Static(path, path)
	}

	router.GET("/watch", func(context *gin.Context) {
		currentUser := watch.getUser(context)
		context.HTML(http.StatusOK, "watch.gohtml", gin.H{
			"currentUser": currentUser,
			"avatars":     user.AvatarValues,
		})
	})

	router.GET("/watch/room/:id", func(context *gin.Context) {
		currentUser := watch.getUser(context)
		context.HTML(http.StatusOK, "room.gohtml", gin.H{
			"currentUser": currentUser,
			"room":        context.Param("id"),
			"avatars":     user.AvatarValues,
		})
	})
}
