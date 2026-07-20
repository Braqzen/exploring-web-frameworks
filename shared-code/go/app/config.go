package app

import (
	"encoding/json"
	"fmt"
	"os"
)

type configJson struct {
	Default   defaultSettingsJson     `json:"default"`
	Overrides map[string]overrideJson `json:"overrides"`
}

type defaultSettingsJson struct {
	Latency          Settings `json:"latency"`
	Error            Settings `json:"error"`
	RequestSizeLimit uint64   `json:"request_size_limit"`
}

type Settings struct {
	Enabled bool  `json:"enabled"`
	Rate    uint8 `json:"rate"`
}

type overrideJson struct {
	Latency          *settingsOverride `json:"latency"`
	Error            *settingsOverride `json:"error"`
	RequestSizeLimit *uint64           `json:"request_size_limit"`
}

type settingsOverride struct {
	Enabled *bool  `json:"enabled"`
	Rate    *uint8 `json:"rate"`
}

type AppConfig struct {
	Latency          Settings
	Error            Settings
	RequestSizeLimit uint64
}

func NewAppConfig() (AppConfig, error) {
	service, ok := os.LookupEnv("SERVICE")
	if !ok || service == "" {
		return AppConfig{}, fmt.Errorf("SERVICE env var is required")
	}

	content, err := os.ReadFile("/app/provider.json")
	if err != nil {
		return AppConfig{}, fmt.Errorf("failed to read /app/provider.json: %w", err)
	}

	var file configJson
	if json.Unmarshal(content, &file) != nil {
		return AppConfig{}, fmt.Errorf("failed to parse provider.json: %w", err)
	}

	if file.Overrides == nil {
		file.Overrides = map[string]overrideJson{}
	}

	settings := file.Default

	providerOverrides, ok := file.Overrides[service]
	if ok {
		if providerOverrides.Latency != nil {
			if providerOverrides.Latency.Enabled != nil {
				settings.Latency.Enabled = *providerOverrides.Latency.Enabled
			}
			if providerOverrides.Latency.Rate != nil {
				settings.Latency.Rate = *providerOverrides.Latency.Rate
			}
		}

		if providerOverrides.Error != nil {
			if providerOverrides.Error.Enabled != nil {
				settings.Error.Enabled = *providerOverrides.Error.Enabled
			}
			if providerOverrides.Error.Rate != nil {
				settings.Error.Rate = *providerOverrides.Error.Rate
			}
		}

		if providerOverrides.RequestSizeLimit != nil {
			settings.RequestSizeLimit = *providerOverrides.RequestSizeLimit
		}
	}

	return AppConfig{
		Latency:          settings.Latency,
		Error:            settings.Error,
		RequestSizeLimit: settings.RequestSizeLimit,
	}, nil
}
