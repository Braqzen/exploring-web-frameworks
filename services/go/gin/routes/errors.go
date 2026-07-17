package routes

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type AppError struct {
	status  int
	message string
}

var AppErrors = struct {
	TaskNotFound    AppError
	InvalidPath     AppError
	InvalidMethod   AppError
	InvalidJsonBody AppError
	Internal        AppError
}{
	TaskNotFound:    AppError{http.StatusNotFound, "Task not found"},
	InvalidPath:     AppError{http.StatusNotFound, "Invalid path"},
	InvalidMethod:   AppError{http.StatusMethodNotAllowed, "Invalid method"},
	InvalidJsonBody: AppError{http.StatusUnprocessableEntity, "Invalid body JSON"},
	Internal:        AppError{http.StatusInternalServerError, "Internal server error"},
}

func (self AppError) Error(c *gin.Context) {
	c.JSON(self.status, gin.H{"error": self.message})
}
