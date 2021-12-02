package main

import (
	"context"
	"fmt"
	grpccli "git.5th.im/lb-public/gear/micro-kit/client/grpc"
	"github.com/micro/go-micro/client"
	"github.com/micro/go-micro/codec"
	"google.golang.org/grpc"
	"time"
)

var _ time.Time
var _ client.Client

func main() {
	grpccli.RegisterCodecs()

	// call with json
	{
		conn, err := grpc.Dial(":50051", grpc.WithInsecure(), grpc.WithDefaultCallOptions(grpc.CallContentSubtype("json")))
		if err != nil {
			fmt.Printf("连接服务端失败: %s", err)
			return
		}
		defer conn.Close()
		var i interface{}

		err = conn.Invoke(context.Background(), "/helloworld.Job/TriggerCreateRebate", map[string]string{"name": "from json"}, &i)
		if err != nil {
			fmt.Printf("Invoke: %s\n", err)
			return
		}

		fmt.Printf("ok %+v\n", i)
	}

	// call with proto
	{
		conn, err := grpc.Dial(":50051", grpc.WithInsecure(), grpc.WithDefaultCallOptions())
		if err != nil {
			fmt.Printf("连接服务端失败: %s", err)
			return
		}
		defer conn.Close()
		var i HelloReply

		err = conn.Invoke(context.Background(), "/helloworld.Job/TriggerCreateRebate", &HelloRequest{
			Name: "from proto",
		}, &i)
		if err != nil {
			fmt.Printf("Invoke: %s\n", err)
			return
		}

		fmt.Printf("ok %+v\n", i)
	}

	// call with lb-go micro
	{
		i := map[string]interface{}{}
		c := grpccli.NewClient()
		req := newRequest("test_service", "/helloworld.Job/TriggerCreateRebate",
			map[string]string{"name": "from lb-go micro"}, "application/json", RequestOptions{
				ContentType: "application/json",
				Stream:      false,
				Context:     nil,
			})
		err := c.Call(context.Background(), req, &i, client.WithAddress(fmt.Sprintf("localhost:50051")), client.WithRequestTimeout(5*time.Minute))
		if err != nil {
			fmt.Printf("Call: %s\n", err)
			return
		}

		fmt.Printf("ok %+v\n", i)
	}
}

type rpcRequest struct {
	service     string
	method      string
	endpoint    string
	contentType string
	codec       codec.Codec
	body        interface{}
	opts        RequestOptions
}
type RequestOptions struct {
	ContentType string
	Stream      bool

	// Other options for implementations of the interface
	// can be stored in a context
	Context context.Context
}

func newRequest(service, endpoint string, request interface{}, contentType string, opts RequestOptions) *rpcRequest {
	// set the content-type specified
	if len(opts.ContentType) > 0 {
		contentType = opts.ContentType
	}

	return &rpcRequest{
		service:     service,
		method:      endpoint,
		endpoint:    endpoint,
		body:        request,
		contentType: contentType,
		opts:        opts,
	}
}

func (r *rpcRequest) ContentType() string {
	return r.contentType
}

func (r *rpcRequest) Service() string {
	return r.service
}

func (r *rpcRequest) Method() string {
	return r.method
}

func (r *rpcRequest) Endpoint() string {
	return r.endpoint
}

func (r *rpcRequest) Body() interface{} {
	return r.body
}

func (r *rpcRequest) Codec() codec.Writer {
	return r.codec
}

func (r *rpcRequest) Stream() bool {
	return false
}
