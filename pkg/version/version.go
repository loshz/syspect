// Package version exposes public constants used to configure service
// data at build time.
package version

// Build represents the current build number of a service.
//
// Use `--ldflags` at build time to set this value.
var Build = "dev"
