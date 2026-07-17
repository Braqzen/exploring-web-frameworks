package app

import (
	"encoding/json"
	"fmt"

	"github.com/google/uuid"
)

func ParseUUID(s string) (uuid.UUID, error) {
	return uuid.Parse(s)
}

func ParseTask(data []byte) (Task, error) {
	var task Task
	err := json.Unmarshal(data, &task)
	if err != nil {
		return Task{}, err
	}
	if task.Secret == "" {
		return Task{}, fmt.Errorf("missing secret")
	}
	err = validateOperation(task.Operation)
	if err != nil {
		return Task{}, err
	}
	return task, nil
}

func ParsePatchedTask(data []byte) (PatchedTask, error) {
	var task PatchedTask
	err := json.Unmarshal(data, &task)
	if err != nil {
		return PatchedTask{}, err
	}
	err = validateOperation(task.Operation)
	if err != nil {
		return PatchedTask{}, err
	}
	return task, nil
}

func validateOperation(op Operation) error {
	switch op {
	case Transform, Sort, Merge, Compute:
		return nil
	default:
		return fmt.Errorf("invalid operation: %s", op)
	}
}
