from __future__ import annotations
import os
from dataclasses import dataclass
from pydantic import BaseModel, Field


class ConfigJson(BaseModel):
    default: DefaultSettingsJson
    overrides: dict[str, OverrideJson] = Field(default_factory=dict)


class DefaultSettingsJson(BaseModel):
    latency: Settings
    error: Settings
    request_size_limit: int


class Settings(BaseModel):
    enabled: bool
    rate: int


class SettingsOverride(BaseModel):
    enabled: bool | None = None
    rate: int | None = None


class OverrideJson(BaseModel):
    latency: SettingsOverride | None = None
    error: SettingsOverride | None = None
    request_size_limit: int | None = None


@dataclass
class Config:
    latency: Settings
    error: Settings
    request_size_limit: int

    @classmethod
    def new(cls) -> Config:
        service = os.environ.get("SERVICE")
        if service is None or service == "":
            raise ValueError("SERVICE env var is required")

        try:
            with open("/app/provider.json", encoding="utf-8") as f:
                content = f.read()
        except OSError as e:
            raise ValueError(f"failed to read /app/provider.json: {e}") from e

        try:
            file = ConfigJson.model_validate_json(content)
        except Exception as e:
            raise ValueError(f"failed to parse provider.json: {e}") from e

        latency = file.default.latency
        error = file.default.error
        request_size_limit = file.default.request_size_limit

        provider_overrides = file.overrides.get(service)
        if provider_overrides is not None:
            if provider_overrides.latency is not None:
                enabled = latency.enabled
                rate = latency.rate

                if provider_overrides.latency.enabled is not None:
                    enabled = provider_overrides.latency.enabled
                if provider_overrides.latency.rate is not None:
                    rate = provider_overrides.latency.rate

                latency = Settings(enabled=enabled, rate=rate)

            if provider_overrides.error is not None:
                enabled = error.enabled
                rate = error.rate

                if provider_overrides.error.enabled is not None:
                    enabled = provider_overrides.error.enabled
                if provider_overrides.error.rate is not None:
                    rate = provider_overrides.error.rate

                error = Settings(enabled=enabled, rate=rate)

            if provider_overrides.request_size_limit is not None:
                request_size_limit = provider_overrides.request_size_limit

        return cls(
            latency=latency,
            error=error,
            request_size_limit=request_size_limit,
        )
