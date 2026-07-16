package telemetry

import (
	"errors"
	"os"

	"github.com/grafana/pyroscope-go"
)

type Profiler struct {
	serviceName string
	client      *pyroscope.Profiler
}

func NewProfiler(serviceName string) *Profiler {
	return &Profiler{serviceName: serviceName}
}

func (self *Profiler) Start() error {
	if self.client != nil {
		return nil
	}

	serverAddress, ok := os.LookupEnv("PYROSCOPE_URL")
	if !ok || serverAddress == "" {
		return errors.New("Missing PYROSCOPE_URL")
	}

	config := pyroscope.Config{
		ApplicationName: self.serviceName,
		ServerAddress:   serverAddress,
		Tags:            map[string]string{"lang": "go"},
	}

	profiler, err := pyroscope.Start(config)
	if err != nil {
		return err
	}

	self.client = profiler
	return nil
}

func (self *Profiler) Shutdown() error {
	if self.client == nil {
		return nil
	}
	return self.client.Stop()
}
