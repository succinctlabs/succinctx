package main

import (
	"io"
)

type Proof interface {
}

type VerifyingKey interface {
	NbPublicWitness() int
	ExportSolidity(w io.Writer) error
}
