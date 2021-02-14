package watch

import (
	"github.com/gin-gonic/gin"
	"github.com/kraxarn/website/config"
	"github.com/kraxarn/website/user"
)

type Watch struct {
	token *config.Token
}

func (watch *Watch) getUser(context *gin.Context) *user.User {
	var currentUser *user.User
	token, err := context.Cookie("user")
	if err == nil {
		currentUser, err = user.NewUserFromToken(token, watch.token)
	}

	if currentUser == nil {
		currentUser = user.NewUser()
	}
	if currentUser != nil {
		currentUser.Refresh(context, watch.token)
	}

	return currentUser
}
