package routes

import "github.com/gin-gonic/gin"

type AppError struct {
	Status  int
	Message string
}

var AppErrors = struct {
	TaskNotFound    AppError
	InvalidPath     AppError
	InvalidMethod   AppError
	InvalidJsonBody AppError
	Internal        AppError
}{
	TaskNotFound:    AppError{404, "Task not found"},
	InvalidPath:     AppError{404, "Invalid path"},
	InvalidMethod:   AppError{405, "Invalid method"},
	InvalidJsonBody: AppError{422, "Invalid body JSON"},
	Internal:        AppError{500, "Internal server error"},
}

func SendError(c *gin.Context, err AppError) {
	c.JSON(err.Status, gin.H{"error": err.Message})
}
