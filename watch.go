package watch

import (
	"github.com/kraxarn/website/config"
	"github.com/kraxarn/website/user"
	"net/http"
)

type Watch struct {
	token *config.Token
}

func (watch *Watch) getUser(writer http.ResponseWriter, request *http.Request) *user.User {
	var currentUser *user.User
	cookie, err := request.Cookie("user")
	if err == nil {
		currentUser, err = user.NewUserFromToken(cookie.Value, watch.token)
	}

	if currentUser == nil {
		currentUser = user.NewUser()
	}
	if currentUser != nil {
		currentUser.Refresh(writer, watch.token)
	}

	return currentUser
}
