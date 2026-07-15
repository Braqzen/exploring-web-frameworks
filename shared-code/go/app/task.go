package app

type Task struct {
	Secret    string    `json:"secret"`
	Operation Operation `json:"operation"`
}

type PatchedTask struct {
	Operation Operation `json:"operation"`
}
