package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"github.com/kraxarn/go-watch/storage"
	"html/template"
	"io/ioutil"
	"net/http"
	"strings"
)

func main() {
	// Create router and add some middleware
	// (using .Default directly generates a warning)
	router := gin.Default()

	// Setup some html functions
	router.SetFuncMap(template.FuncMap{
		"userImages": func() []string {
			return storage.Images
		},
		"getUserImageName": func(key string) string {
			name := storage.ImageNames[key]
			return strings.ToUpper(name[0:1]) + name[1:]
		},
	})

	// Add all files in html folder as templates
	router.LoadHTMLGlob("html/*.html")

	// Add all folders and files in static folder
	staticFiles, _ := ioutil.ReadDir("static")
	for _, file := range staticFiles {
		path := fmt.Sprintf("static/%v", file.Name())
		if file.IsDir() {
			router.Static(file.Name(), path)
		} else {
			router.StaticFile(file.Name(), path)
		}
	}

	// Show index when loading root
	router.GET("", func(context *gin.Context) {
		context.HTML(http.StatusOK, "index.html", nil)
	})

	// When page is not found, redirect page to home
	router.NoRoute(func(context *gin.Context) {
		context.Redirect(http.StatusFound, "/")
	})

	// Start listening on port 8080
	if err := router.Run("localhost:5000"); err != nil {
		fmt.Println(err)
	}
}