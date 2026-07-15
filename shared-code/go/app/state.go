package app

import (
	"sync"

	"github.com/google/uuid"
)

type AppState struct {
	Mu    sync.RWMutex
	Tasks map[uuid.UUID]Task
}

func NewState() *AppState {
	return &AppState{
		Tasks: make(map[uuid.UUID]Task),
	}
}
