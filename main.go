package watch

import (
	"fmt"
	"github.com/kraxarn/website/config"
	"github.com/kraxarn/website/user"
	"html/template"
	"net/http"
)

type Model struct {
	CurrentUser *user.User
	Avatars     []user.Avatar
	Room        string
}

func Route(token *config.Token) error {
	// Main state handler
	watch := Watch{
		token: token,
	}

	// Static files
	for _, path := range []string{
		"css", "img", "js",
	} {
		http.Handle(path, http.FileServer(http.Dir(fmt.Sprintf("static/%s", path))))
	}

	templates, err := template.ParseGlob("../go-watch/html/*.gohtml")
	if err != nil {
		return err
	}

	http.HandleFunc("/watch", func(writer http.ResponseWriter, request *http.Request) {
		currentUser := watch.getUser(writer, request)
		err := templates.Lookup("watch.gohtml").Execute(writer, Model{
			CurrentUser: currentUser,
			Avatars:     user.AvatarValues,
		})
		if err != nil {
			fmt.Printf("/watch filed: %v\n", err)
		}
	})

	http.HandleFunc("/watch/room/:id", func(writer http.ResponseWriter, request *http.Request) {
		currentUser := watch.getUser(writer, request)
		err := templates.Lookup("room.gohtml").Execute(writer, Model{
			CurrentUser: currentUser,
			Avatars:     user.AvatarValues,
			Room:        request.URL.Query().Get("id"),
		})
		if err != nil {
			fmt.Printf("/watch/room filed: %v\n", err)
		}
	})

	return nil
}
