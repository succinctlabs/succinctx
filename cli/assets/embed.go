package assets

import "embed"

// To use this, add all assets inside this directory and export them via the go:embed directive.
// All of them are available to other packages as embed.FS variables.

//go:embed circuit.tmpl
var Circuit embed.FS

//go:embed main.tmpl
var Main embed.FS
