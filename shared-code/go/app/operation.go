package app

type Operation string

const (
	Transform Operation = "Transform"
	Sort      Operation = "Sort"
	Merge     Operation = "Merge"
	Compute   Operation = "Compute"
)
