package dev

import (
	"bytes"
	"compress/zlib"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"testing"
)

type Service struct {
	Name      string            `json:"name"`
	Version   string            `json:"version"`
	Metadata  map[string]string `json:"metadata"`
	Endpoints []*Endpoint       `json:"endpoints"`
	Nodes     []*Node           `json:"nodes"`
}

type Node struct {
	Id       string            `json:"id"`
	Address  string            `json:"address"`
	Port     int               `json:"port"`
	Metadata map[string]string `json:"metadata"`
}

type Endpoint struct {
	Name     string            `json:"name"`
	Request  *Value            `json:"request,omitempty"`
	Response *Value            `json:"response,omitempty"`
	Metadata map[string]string `json:"metadata,omitempty"`
}

type Value struct {
	Name   string   `json:"name"`
	Type   string   `json:"type"`
	Values []*Value `json:"values"`
}

func encodeEndpoints(en []*Endpoint) []string {
	var tags []string
	for _, e := range en {
		if b, err := json.Marshal(e); err == nil {
			fmt.Printf("%s", b)
			tags = append(tags, "e-"+encode(b))
		}
	}
	return tags
}

func decode(d string) []byte {
	hr, err := hex.DecodeString(d)
	if err != nil {
		return nil
	}

	br := bytes.NewReader(hr)
	zr, err := zlib.NewReader(br)
	if err != nil {
		return nil
	}

	rbuf, err := ioutil.ReadAll(zr)
	if err != nil {
		return nil
	}

	return rbuf
}
func encode(buf []byte) string {
	var b bytes.Buffer
	defer b.Reset()

	w := zlib.NewWriter(&b)
	if _, err := w.Write(buf); err != nil {
		return ""
	}
	w.Close()

	return hex.EncodeToString(b.Bytes())
}

func TestEncodeEndpoints(t *testing.T) {
	t.Logf("%s", encodeEndpoints([]*Endpoint{
		{
			Name:     "Helloworld.SayHello",
			Request:  nil,
			Response: nil,
			Metadata: nil,
		},
	}))
}

func TestDecode(t *testing.T) {
	t.Logf("%s", decode("789cab56ca4bcc4d55b252f248cdc9c92fcf2fca49d10f4eac04f3946a01a2aa0ac8"))
	t.Logf("%s", decode("789caa562a4e2d2a4b2d52b2522a2a4856aa05040000ffff30dd0597"))
	t.Logf("%s", decode("789caa562a4a4dcf2c2e29aa54b2524acecf2b2ecd51aa05040000ffff560f07c8"))
	t.Logf("%s", decode("789caa562a28ca2fc94fcecf51b252ca2d4d2e50aa05040000ffff46a906e2"))
	t.Logf("%s", decode("789caa562a294acc2b2ec82f2a51b252ca28292950aa05040000ffff4f930768"))
	t.Logf("%s", decode("789caa564a2acacf4e2d52b252ca28292950aa05040000ffff364d0600"))
}