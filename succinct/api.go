package succinct

import (
	"github.com/consensys/gnark/frontend"
)

type API struct {
	api frontend.API
}

func NewAPI(api frontend.API) *API {
	return &API{api: api}
}

func (a *API) API() frontend.API {
	return a.api
}
