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

func (p *Profiler) Start() error {
	if p.client != nil {
		return nil
	}

	serverAddress, ok := os.LookupEnv("PYROSCOPE_URL")
	if !ok || serverAddress == "" {
		return errors.New("Missing PYROSCOPE_URL")
	}

	config := pyroscope.Config{
		ApplicationName: p.serviceName,
		ServerAddress:   serverAddress,
		Tags:            map[string]string{"lang": "go"},
	}

	profiler, err := pyroscope.Start(config)
	if err != nil {
		return err
	}

	p.client = profiler
	return nil
}

func (p *Profiler) Shutdown() error {
	if p.client == nil {
		return nil
	}
	return p.client.Stop()
}
